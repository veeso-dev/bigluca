require("dotenv").config();
const { NFTStorage, File } = require("nft.storage");
const fs = require("fs");
const mime = require("mime");
const path = require("path");

const NFT_STORAGE_TOKEN = process.env.APIKEY;

if (!NFT_STORAGE_TOKEN) {
  throw new Error("APIKEY unset in environment");
}

/**
 * A helper to read a file from a location on disk and return a File object.
 * Note that this reads the entire file into memory and should not be used for
 * very large files.
 * @param {string} filePath the path to a file to store
 * @returns {File} a File object containing the file content
 */
function fileFromPath(filePath) {
  const content = fs.readFileSync(filePath);
  const type = mime.getType(filePath);
  return new File(
    [content],
    `${path.basename(filePath)}.${path.extname(filePath)}`,
    { type }
  );
}

/**
 * Reads an image file from `imagePath` and stores an NFT with the given name and description.
 * @param {any} metadata
 * @param {string} imagePath the path to an image file
 * @returns {any}
 */
async function storeNFT(metadata, imagePath) {
  // load the file from disk
  const image = fileFromPath(imagePath);

  // create a new NFTStorage client using our API key
  const nftstorage = new NFTStorage({ token: NFT_STORAGE_TOKEN });

  // store image
  const imageCid = await nftstorage.storeBlob(image);
  const imageUrl = `ipfs://${imageCid}`;

  metadata.image = imageUrl;

  const metadataCid = await nftstorage.storeBlob(
    new File([JSON.stringify(metadata)], "metadata.json", { type: "json" })
  );
  // call client.store, passing in the image & metadata
  return {
    image: imageCid,
    imageUrl,
    metadata: metadataCid,
    metadataUrl: `ipfs://${metadataCid}`,
  };
}

async function run(metadataFilename, imageFilename, outputDir) {
  // load metadata
  const data = fs.readFileSync(metadataFilename, { encoding: "utf8" });
  let metadata = JSON.parse(data);
  const outputFile = `${outputDir}/${path.basename(metadataFilename)}`;

  const nft = await storeNFT(metadata, imageFilename);
  // write output
  fs.writeFileSync(outputFile, JSON.stringify(nft, null, 2));
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
