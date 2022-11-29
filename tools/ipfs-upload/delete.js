require("dotenv").config();
const { NFTStorage } = require("nft.storage");
const fs = require("fs");

const NFT_STORAGE_TOKEN = process.env.APIKEY;

if (!NFT_STORAGE_TOKEN) {
  throw new Error("APIKEY unset in environment");
}

/**
 * Reads an image file from `imagePath` and stores an NFT with the given name and description.
 * @param {any} metadata
 * @param {string} imagePath the path to an image file
 * @returns {any}
 */
async function deleteCid(cid) {
  const nftstorage = new NFTStorage({ token: NFT_STORAGE_TOKEN });

  return await nftstorage.delete(cid);
}

async function run(metadataFilename) {
  // load metadata
  const data = fs.readFileSync(metadataFilename, { encoding: "utf8" });
  const metadata = JSON.parse(data);

  if (metadata.ipnft) {
    await deleteCid(metadata.ipnft);
  }
  if (metadata.metadata) {
    await deleteCid(metadata.metadata);
  }
  if (metadata.image) {
    await deleteCid(metadata.image);
  }
  fs.unlinkSync(metadataFilename);
}

// load arguments
if (process.argv.length < 3) {
  console.error("Usage: <metadataFilename>");
  process.exit(255);
}
const metadataFilename = process.argv[2];

run(metadataFilename)
  .then(() => {
    console.log("OK!");
  })
  .catch((err) => {
    console.error(`process failed: ${err.message}`);
    process.exit(1);
  });
