// app/api/chains.ts
export async function GET() {
  const chains = [
    {
      name: "NeuroGenesisChain",
      status: "active",
      created_at: "2025-04-07"
    },
    {
      name: "MetaSynapseNet",
      status: "pending",
      created_at: "2025-04-06"
    }
  ];

  return Response.json(chains);
}
