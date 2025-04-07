// app/api/tokens.ts
export async function GET() {
  const tokens = [
    {
      name: "Auroraium",
      supply: 750,
      deployed: true
    },
    {
      name: "NeuroFuel",
      supply: 420000,
      deployed: false
    }
  ];

  return Response.json(tokens);
}
