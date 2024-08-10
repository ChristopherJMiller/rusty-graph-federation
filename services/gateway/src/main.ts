import { ApolloGateway, IntrospectAndCompose } from "@apollo/gateway";
import { ApolloServer } from "@apollo/server";
import { startStandaloneServer } from "@apollo/server/standalone";


async function main() {
  const gateway = new ApolloGateway({
    supergraphSdl: new IntrospectAndCompose({
      subgraphs: [
        { name: "profile", url: "http://localhost:8000/" }
      ]
    })
  });

  const server = new ApolloServer({ gateway });

  const { url } = await startStandaloneServer(server, {
    listen: {
      path: "0.0.0.0",
      port: 4000
    }
  });
  console.log(`Server running at ${url}`);
}

main();