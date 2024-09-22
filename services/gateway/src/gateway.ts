import { ApolloGateway, IntrospectAndCompose } from "@apollo/gateway";
import { DiscoverySubgraphs } from "./defs";
import fetchRetry from "fetch-retry";

const fetchWithRetry = fetchRetry(fetch);

const discoveryUrl = process.env.DISCOVERY_URL;

if (!discoveryUrl) {
  throw "DISCOVERY_URL not provided";
}

export async function getDiscoveredSubgraphs(): Promise<DiscoverySubgraphs> {
  const res = await fetchWithRetry(discoveryUrl, {
    retries: 3,
    retryDelay: 3000,
  });

  if (res.status !== 200) {
    throw `Failed to contact discovery endpoint. Status ${res.status} output ${await res.text()}`;
  }

  const subgraphs = (await res.json()) as DiscoverySubgraphs;

  return subgraphs;
}

export async function buildGateway(): Promise<
  [DiscoverySubgraphs, ApolloGateway]
> {
  const subgraphs = await getDiscoveredSubgraphs();

  return [
    subgraphs,
    new ApolloGateway({
      supergraphSdl: new IntrospectAndCompose({
        subgraphs,
      }),
    }),
  ];
}
