var express = require("express");
var app = express();
const path = require("path");
const fs = require("fs");
const cors = require('cors');
const https = require("https");



const SERVER_PORT = 12121;

app.use(cors({
    origin: '*'
}));

app.get("/GetVideoKey", (req, res, next) => {
    var key = "";
    try {
        key = req.query.key;
    } catch (err) {}

    res.header("Content-Type", "Application/octet-stream");
    res.send(Buffer.from(key, "base64url"));
});

https
  .createServer(
    {
        key: fs.readFileSync(path.resolve(__dirname, './certificates/server.key')),
        cert: fs.readFileSync(path.resolve(__dirname, './certificates/server.crt'))
    },
    app)
  .listen(SERVER_PORT, ()=>{
    console.log(`Echo server is running on port ${SERVER_PORT}`);
  });


// app.listen(SERVER_PORT, () => {
//     console.log(`Echo server is running on port ${SERVER_PORT}`);
// });
