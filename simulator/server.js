const express = require("express");
const path = require("path");
const http = require("http");

const app = express();
const PORT = 3000;
const BOX_HOST = "localhost";
const BOX_PORT = 9989;

// Serve the Box server's turubai JS runtime (imports.js, etc.)
app.use("/turubai", express.static(path.join(__dirname, "../../box/Server/wwwroot/turubai")));

app.use(express.static(path.join(__dirname)));

// Proxy Box API calls so imports.js can reach /api/user/... and /api/user/display-name
app.use("/api", (req, res) => {
    const options = {
        hostname: BOX_HOST,
        port: BOX_PORT,
        path: "/api" + req.url,
        method: req.method,
        headers: req.headers,
    };
    const proxy = http.request(options, (proxyRes) => {
        res.writeHead(proxyRes.statusCode, proxyRes.headers);
        proxyRes.pipe(res, { end: true });
    });
    proxy.on("error", () => res.status(502).send("Box server unreachable"));
    req.pipe(proxy, { end: true });
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
