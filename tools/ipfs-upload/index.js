require("dotenv").config();
const { NFTStorage, File } = require("nft.storage");
const fs = require("fs");

const NFT_STORAGE_TOKEN = process.env.APIKEY;

if (!NFT_STORAGE_TOKEN) {
  throw new Error("APIKEY unset in environment");
}

async function run(metadataFilename, imageFilename, outputDir) {
  // load metadata
  const data = fs.readFileSync(metadataFilename, { encoding: "utf8" });
  let metadata = JSON.parse(data);
  // check if image is set
  if (metadata.image) {
    console.error(
      "metadata.image is set. Clear field if you want to upload file to IPFS"
    );
    process.exit(1);
  }
  // load output file
  const outputFile = `${outputDir}/${metadata.name
    .replace(" ", "-")
    .replace(/^.*[\\\/]/, "")}.json`;
  // load image
  const imageData = fs.readFileSync(imageFilename);

  // load client
  const client = new NFTStorage({ token: NFT_STORAGE_TOKEN });
  // load image
  const image = new File([imageData], "image.png", { type: "image/png" });
  metadata.image = image;
  metadata = await client.store(metadata);
  fs.writeFileSync(outputFile, JSON.stringify(metadata, null, 2));
}

// load arguments
if (process.argv.length < 5) {
  console.error("Usage: <metadataFilename> <imageFilename> <output_dir>");
  process.exit(255);
}
const metadataFilename = process.argv[2];
const imageFilename = process.argv[3];
const outputDir = process.argv[4];

run(metadataFilename, imageFilename, outputDir)
  .then(() => {
    console.log("OK!");
  })
  .catch((err) => {
    console.error(`process failed: ${err.message}`);
    process.exit(1);
  });
