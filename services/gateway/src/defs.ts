export interface ServiceEndpoint {
  name: string;
  url: string;
}

export type DiscoverySubgraphs = ServiceEndpoint[];

export function areDiscoverySubgraphsEqual(
  a: DiscoverySubgraphs,
  b: DiscoverySubgraphs
): boolean {
  // Assumes the discovery endpoint is consistent with ordering
  return JSON.stringify(a) === JSON.stringify(b);
}

export function delay(ms: number): Promise<void> {
  return new Promise(function (resolve, _reject) {
    setTimeout(function () {
      resolve();
    }, ms);
  });
}
