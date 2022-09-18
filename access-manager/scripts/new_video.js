require("dotenv").config();

const { SecretNetworkClient, Wallet } = require("secretjs");

const printObj = (obj) => {
  console.log(JSON.stringify(obj, null, 2));
};

(async () => {
  const wallet = new Wallet(
    "grant rice replace explain federal release fix clever romance raise often wild taxi quarter soccer fiber love must tape steak together observe swap guitar"
  );
  const client = await SecretNetworkClient.create({
    grpcWebUrl: process.env.GRPCWEB_URL,
    chainId: process.env.CHAIN_ID,
    wallet: wallet,
    walletAddress: wallet.address,
  });

  let res = await client.tx.compute.executeContract(
    {
      contractAddress: process.env.ACCESS_MANAGER_ADDRESS,
      msg: {
        new_video: {
          name: "tom",
          royalty_info: {
            decimal_places_in_rates: 3,
            royalties: [{ recipient: wallet.address, rate: 500 }],
          },
          image_url: "ipfs://QmaK5Y969GeFqcBmu5BAPWgXfwkU9hpQCYJRJyQdYtCBjz",
          video_url:
            "ipfs://QmVbKFQRNx166RuqyyEb8XMwnw7GY57g7sXgcmxxKrL9ms/main.m3u8",
          decryption_key: "UainRqKrHz_62Gfx0Qv4Hg",
          price: { token: { native: "uscrt" }, amount: "10000000" },
        },
      },
      sender: wallet.address,
    },
    { gasLimit: 500_000 }
  );
  if (!res.arrayLog) {
    console.log(`Error: ${res.rawLog}`);
    process.exit(1);
  }
  res = res.arrayLog.find((l) => l.key === "new_video_id");
  printObj(res);
})();
