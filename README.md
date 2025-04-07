first:

```
cd poem-server
cargo build
```

to build the server that will be ran to host static files and also view logs coming from mobile. next:

```
cd ../wasm
cargo build --relase --target wasm32-unknown-unknown
```

to build a .wasm file. next

```
cd ..
npm run host
```

will build the frontend static files and copy over the .wasm file to dist/ and will also run the server built in step 1.

finally, open phone, go to `https://{yourip}:3002` and click the button which will:

1. prompt browser to pick a directory
2. once directory selected, it will list every .png file
3. every png file will be read as an arraybuffer which will be passed to wasm
4. wasm will (today) simply return the width of the image
5. browser will sendLog the width to the server so you can read it and validate it

