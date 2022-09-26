# Echo Server

### Usage

Create certificate to run the server as HTTPs (to avoid mixed content and better support in mobile devices).  
The certificate should be saved unser "certificates" folder.

```bash
openssl genrsa 2048 > server.key
chmod 400 server.key
openssl req -new -x509 -nodes -sha256 -days 365 -key server.key -out server.crt
```

Install dependencies and run:

```bash
npm install
node index.js
```
