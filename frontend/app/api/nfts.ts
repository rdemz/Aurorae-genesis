// app/api/nfts.ts
export async function GET() {
  const nfts = [
    {
      title: "Aurora-Swarm",
      description: "Essaim autonome de micro-IA interconnectées",
      image_url: "https://arweave.net/nft1.png",
      minted: false
    },
    {
      title: "Dream-Fabricator",
      description: "Entité générative de visions IA",
      image_url: "https://arweave.net/nft2.png",
      minted: true
    }
  ];

  return Response.json(nfts);
}
