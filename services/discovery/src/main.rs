use std::{
    collections::{BTreeMap, HashMap},
    convert::Infallible,
    sync::LazyLock,
};

use anyhow::Result;
use futures::prelude::*;
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::Api,
    runtime::{watcher, watcher::Config, WatchStreamExt},
    Client,
};
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::{info, instrument};
use warp::Filter;

const DISCOVERY_ANNOTATION_PORT: &'static str = "gateway.chrismiller.xyz/port";

#[derive(Serialize)]
struct SubgraphService {
    pub name: String,
    pub url: String,
}

static WATCH_STATE: LazyLock<RwLock<HashMap<String, SubgraphService>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

async fn display_subgraph() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(
        &WATCH_STATE.read().await.values().collect::<Vec<_>>(),
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Connecting to K8s");
    let client = Client::try_default().await?;
    let api = Api::<Service>::default_namespaced(client);

    tokio::spawn(async {
        let routes = warp::path("subgraphs").and_then(display_subgraph);
        info!("Opening discovery endpoint at port 3030");
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await
    });

    info!("Registering Watcher");
    watcher(api, Config::default())
        .applied_objects()
        .default_backoff()
        .try_for_each(|service| async move {
            if let Some((subgraph, port)) =
                extract_discovery_annotations(&service.metadata.name, &service.metadata.annotations)
            {
                info!("Discovered {subgraph}:{port}, writing.");
                WATCH_STATE.write().await.insert(
                    subgraph.clone(),
                    SubgraphService {
                        name: subgraph,
                        url: format!("http://{}:{}", service.metadata.name.unwrap(), port),
                    },
                );
            // If it can't build a valid subgraph service from it, double check if it needs to remove it from the watch state
            } else if let Some(service_name) = service.metadata.name {
                if WATCH_STATE.read().await.contains_key(&service_name) {
                    WATCH_STATE.write().await.remove(&service_name);
                }
            }
            Ok(())
        })
        .await?;

    Ok(())
}

#[instrument(skip(annotations), ret)]
fn extract_discovery_annotations(
    name: &Option<String>,
    annotations: &Option<BTreeMap<String, String>>,
) -> Option<(String, String)> {
    if let Some(annotations) = annotations {
        return match (name, annotations.get(DISCOVERY_ANNOTATION_PORT)) {
            (Some(service), Some(port)) => Some((service.clone(), port.clone())),
            _ => None,
        };
    } else {
        return None;
    }
}
