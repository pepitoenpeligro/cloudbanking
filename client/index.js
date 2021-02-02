const express = require("express")
const http = require("http")
const path = require("path")
const app = express()


app.use(express.static(path.join(__dirname, "build")))

app.get("/", (req, res) => {
	res.header("Access-Control-Allow-Origin", "pepecordoba.site")
	res.sendFile(path.join(__dirname, "build", "index.html"))
})

const server = http.createServer(app)
const PORT = process.env.port || 8080

server.listen(PORT)
server.on("listening", () => {
	console.log(`Servidor escuchando en el puerto: ${PORT}`)
})