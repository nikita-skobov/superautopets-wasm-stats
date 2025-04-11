
# super auto pets stats

This is a project to count my [super auto pets](https://teamwoodgames.com/) wins and show various statistics.

![superautopetsstats](https://github.com/user-attachments/assets/9392381b-5014-4603-8a61-e38f18e4abe7)

## why?

I take a screenshots for every time i win a game of super auto pets. the game itself does have some statistics but i wanted more granularity. I achieve this by writing a webapp which can read all of my screenshots and extract information from the screenshots to provide statistics such as:

- how many lives did I have at the time of the W
- what the turn count was
- the time/date of the W

- etc.

## requirements

I wanted this to be a website such that I don't need to transfer all of my files to my computer every time i want to check my stats; i can simply open a website and select my screenshots folder

## how it works

The website prompts for a directory using the experimental [file system API](https://developer.mozilla.org/en-US/docs/Web/API/File_System_API), and from there it loads each file, passing it to a web assembly program which uses image processing to extract regions of the screenshot and returns a single `i64` value which encodes the number of hearts, whether or not there was a bandage, and the turn count.


## development

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

