# bigluca

<p align="center">
  <img src="bigluca.png" width="256" />
</p>

<p align="center">~ Cryptopapies NFT generator ~</p>

[![ci](https://github.com/cryptopapies/bigluca/workflows/Build/badge.svg)](https://github.com/cryptopapies/bigluca/actions)

---

## Get started 🚀

What you'll need:

- Rust
- NodeJS

### Build Big Luca

```sh
cargo build -r
```

### Generate NFT

```sh
cargo run -r -- --config config/config.json --database-path database/db.json --collection COLLECTION_NAME --output artifacts/COLLECTION_NAME -n AMOUNT_OF_NFTS_TO_GENERATE -v
```

Collection names:

- dubai-papi

---

## Upload NFT to IPFS

Once the NFT are generated, they will be located at `artifacts/collection-name/` and there will be two files for each NFT, a JSON file with the metadata and a PNG file with the image.

Now enter `tools/ipfs-upload` and run:

```node index.js <PATH_TO_JSON> <PATH_TO_PNG> output/collection_name```

or use the bash script to upload many automatically:

```./deploy.sh ../../artifacts/dubai-papi/ output/dubai-papi/ START_INDEX END_INDEX```

---

## License 📃

(c) Copyright 2022 Christian Visintin, all rights reserved.
