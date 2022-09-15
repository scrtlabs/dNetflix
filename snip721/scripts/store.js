require("dotenv").config();

const fs = require("fs");
const { SecretNetworkClient, Wallet } = require("secretjs");

const PATH = "access_manager.wasm";

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

  // Read file to byte array
  const file = fs.readFileSync(PATH);
  const fileBytes = new Uint8Array(file);

  let res = await client.tx.compute.storeCode(
    {
      wasmByteCode: fileBytes,
      sender: wallet.address,
      builder: "",
      source: "",
    },
    { gasLimit: 2_000_000 }
  );
  res = res.arrayLog.find((l) => l.key === "code_id");
  const codeId = parseInt(res.value);
  const codeHash = await client.query.compute.codeHash(codeId);
  printObj({ codeId, codeHash: codeHash.toString("hex") });
})();
