# mobile-video

## Build Setup

```bash
# install dependencies
$ npm install
```

Create certificate to run the client as HTTPs (to avoid mixed content and better support in mobile devices).  
The certificate should be saved unser "certificates" folder.

```bash
openssl genrsa 2048 > server.key
chmod 400 server.key
openssl req -new -x509 -nodes -sha256 -days 365 -key server.key -out server.crt
```

```bash
# serve with hot reload at localhost:3000
$ npm run dev

# build for production and launch server
$ npm run build
$ npm run start

# generate static project
$ npm run generate
```

### Video Files

We have an example HLS video encoded with AES-128 uploaded to 
```bash
ipfs://QmVbKFQRNx166RuqyyEb8XMwnw7GY57g7sXgcmxxKrL9ms/main.m3u8  
```
Make sure that you have IPFS gateway with pinning for that IPFS directory to allow smooth video playback.  
Please rename .env.example to .env and change it if needed.  
**Make sure that your IPFS Gateway supports CORS**  
The variable NUXT_ENV_NFT_CONTRACT contains address for NFT contract in the testnet (pulsar-2).
