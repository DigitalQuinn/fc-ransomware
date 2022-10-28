const express = require("express");
const app = express();

const chumps = {};

app.get("/token", (req, res) => { // Generates 4 random numbers
  const token = [
    Math.floor(Math.random() * 10),
    Math.floor(Math.random() * 10),
    Math.floor(Math.random() * 10),
    Math.floor(Math.random() * 10)
  ].join("");

  chumps[token] = req.query.password; // Save numbers to memory
  return res.end(token); // Return the token
});

app.get("/password", (req, res) => {
  const token = req.query.token;
  res.end(chumps[token]);
});

app.listen(3000, () => {
  console.log("started server on port 3000");
  console.log("--------------------------");
});
