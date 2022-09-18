require("dotenv").config();

const fs = require("fs");
const { SecretNetworkClient, Wallet } = require("secretjs");

const PATH = "access_manager.wasm.gz";

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
  if (!res.arrayLog) {
    console.log(`Error: ${res.rawLog}`);
    process.exit(1);
  }
  res = res.arrayLog.find((l) => l.key === "code_id");
  printObj(res);
  const codeId = parseInt(res.value);

  res = await client.tx.compute.instantiateContract(
    {
      codeId,
      initMsg: {
        access_token_wasm: {
          code_id: parseInt(process.env.SNIP721_CODE_ID),
          hash: process.env.SNIP721_CODE_HASH,
        },
        entropy: "Cg==",
      },
      label: Math.random().toString(),
      sender: wallet.address,
    },
    { gasLimit: 500_000 }
  );
  if (!res.arrayLog) {
    console.log(`Error: ${res.rawLog}`);
    process.exit(1);
  }
  res = res.arrayLog.find(
    (l) => l.key === "contract_address" && l.type === "instantiate"
  );
  const addr = res.value;
  printObj({ address: addr });
})();
