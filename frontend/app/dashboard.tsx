import { Card, CardContent } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useEffect, useState } from "react";

export default function Dashboard() {
  const [tokens, setTokens] = useState([]);
  const [nfts, setNfts] = useState([]);
  const [chains, setChains] = useState([]);
  const [iaModules, setIaModules] = useState([]);

  useEffect(() => {
    fetch("/api/tokens").then(res => res.json()).then(setTokens);
    fetch("/api/nfts").then(res => res.json()).then(setNfts);
    fetch("/api/chains").then(res => res.json()).then(setChains);
    fetch("/api/modules").then(res => res.json()).then(setIaModules);
  }, []);

  return (
    <div className="min-h-screen bg-black text-white p-6 font-mono animate-fade-in">
      <h1 className="text-4xl font-bold mb-6 text-center">🌌 AURORAE++ Dashboard</h1>

      <Tabs defaultValue="tokens" className="w-full">
        <TabsList className="grid grid-cols-4 w-full bg-gray-800 rounded-2xl">
          <TabsTrigger value="tokens">ERC20 Tokens</TabsTrigger>
          <TabsTrigger value="nfts">NFT</TabsTrigger>
          <TabsTrigger value="chains">Chains</TabsTrigger>
          <TabsTrigger value="modules">IA Modules</TabsTrigger>
        </TabsList>

        <TabsContent value="tokens">
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mt-6">
            {tokens.map((token, i) => (
              <Card key={i} className="bg-gradient-to-br from-cyan-900 to-black border border-cyan-500">
                <CardContent className="p-4">
                  <h2 className="text-xl font-bold">{token.name}</h2>
                  <p>Supply: {token.supply}</p>
                  <p>Deployed: {token.deployed ? "Yes" : "No"}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="nfts">
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mt-6">
            {nfts.map((nft, i) => (
              <Card key={i} className="bg-gradient-to-br from-purple-800 to-black border border-purple-400">
                <CardContent className="p-4">
                  <h2 className="text-xl font-bold">{nft.title}</h2>
                  <p className="italic">{nft.description}</p>
                  <img src={nft.image_url} alt={nft.title} className="w-full h-48 object-cover mt-2 rounded-xl" />
                  <p className="text-sm text-right mt-2">Minted: {nft.minted ? "Yes" : "No"}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="chains">
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mt-6">
            {chains.map((chain, i) => (
              <Card key={i} className="bg-gradient-to-br from-emerald-800 to-black border border-emerald-400">
                <CardContent className="p-4">
                  <h2 className="text-xl font-bold">{chain.name}</h2>
                  <p>Status: {chain.status}</p>
                  <p>Created: {chain.created_at}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="modules">
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mt-6">
            {iaModules.map((mod, i) => (
              <Card key={i} className="bg-gradient-to-br from-indigo-800 to-black border border-indigo-400">
                <CardContent className="p-4">
                  <h2 className="text-xl font-bold">{mod.name}</h2>
                  <p>Purpose: {mod.purpose}</p>
                  <p>Created: {mod.created_at}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
