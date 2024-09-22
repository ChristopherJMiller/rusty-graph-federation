import { ApolloServer } from "@apollo/server";
import { buildGateway, getDiscoveredSubgraphs } from "./gateway";
import express from "express";
import { expressMiddleware } from "@apollo/server/express4";
import cors from "cors";
import type { Server } from "http";
import { delay, areDiscoverySubgraphsEqual } from "./defs";

// Global variables
let activeGraphqlServer: Server | undefined = undefined;
let activeLivenessServer: Server | undefined = undefined;

let run = true;

async function startGraphqlServer() {
  const app = express();

  console.log("Building gateway");
  const [discoveredSubgraphs, gateway] = await buildGateway();

  const server = new ApolloServer({ gateway });
  await server.start();

  app.use(
    "/graphql",
    cors<cors.CorsRequest>(),
    express.json(),
    expressMiddleware(server)
  );

  app.get("/ready", (_req, res) => {
    res.status(200).send("READY");
  });

  activeGraphqlServer = app.listen({ port: 4000 }, () => {
    console.log(`Server running at port 4000`);
    console.log(`Schema:`, discoveredSubgraphs);
  });

  // The server should be stopped and cleaned up with the gateway no longer matches discovery,
  // which will perform a reboot
  let serverActive = true;
  while (run && serverActive) {
    await delay(5000);
    let restart = false;
    try {
      const possibleNewSubgraph = await getDiscoveredSubgraphs();

      if (
        !areDiscoverySubgraphsEqual(possibleNewSubgraph, discoveredSubgraphs)
      ) {
        console.log("A new subgraph definition is available, restarting...");
        restart = true;
      }
    } catch (e) {
      console.error(
        "An error occurred while trying to check the discovery endpoint:"
      );
      console.error(e);
    }

    if (restart) {
      serverActive = false;
      await new Promise<void>((resolve, _rej) =>
        activeGraphqlServer.close(() => {
          activeGraphqlServer = undefined;
          resolve();
        })
      );
    }
  }
}

async function livenessEndpoint() {
  const app = express();
  app.get("/", (_req, res) => {
    res.status(200).send("OK");
  });

  activeLivenessServer = app.listen({ port: 4001 }, () => {
    console.log(`Liveness Endpoint running at port 4001`);
  });
}

async function main() {
  console.log("Starting Gateway Endpoint");

  livenessEndpoint();

  // Continue to run the graphql server, as it will only exit when
  // the discovery endpoint has changed.
  while (run) {
    await startGraphqlServer();
  }

  if (activeGraphqlServer) {
    activeGraphqlServer.close();
  }

  if (livenessEndpoint) {
    activeLivenessServer.close();
  }
}

process.on("SIGTERM", () => {
  console.log("SIGTERM signal received");
  run = false;
});

main();
