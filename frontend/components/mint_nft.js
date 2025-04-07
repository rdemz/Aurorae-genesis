// mint_nft.js
// Script frontend pour mint un NFT vivant généré par AURORAE++ via Metamask

import { ethers } from "ethers";
import auroraeNftAbi from "./AuroraeNFT.json";

const CONTRACT_ADDRESS = "0xTON_ADRESSE_CONTRAT_NFT";

export async function mintNft(metadataUrl) {
  try {
    if (!window.ethereum) throw new Error("🦊 Metamask non détecté");

    await window.ethereum.request({ method: "eth_requestAccounts" });
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();
    const contract = new ethers.Contract(CONTRACT_ADDRESS, auroraeNftAbi, signer);

    const tx = await contract.mint(metadataUrl);
    console.log("⛏️ Transaction envoyée :", tx.hash);

    const receipt = await tx.wait();
    console.log("✅ NFT minté ! Bloc :", receipt.blockNumber);
  } catch (err) {
    console.error("❌ Erreur de mint :", err);
  }
}
