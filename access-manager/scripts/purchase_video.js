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
        purchase_video: {
          video_id: 1,
        },
      },
      sender: wallet.address,
      sentFunds: [{ denom: "uscrt", amount: "10000000" }],
    },
    { gasLimit: 500_000 }
  );
  res = res.arrayLog.find((l) => l.key === "minted");
  printObj(res);
})();
