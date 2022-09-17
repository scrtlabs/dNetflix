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

  let res = await client.query.compute.queryContract({
    contractAddress: process.env.ACCESS_MANAGER_ADDRESS,
    query: { list_videos: { page: 0, page_size: 100 } },
  });
  printObj(res);
})();
