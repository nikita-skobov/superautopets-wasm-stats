import { useCallback, useEffect, useState } from 'react'
import './App.css'

async function getDir(setDirHandle: (h: any) => void) {
  // @ts-ignore
  const dirHandle = await window.showDirectoryPicker();
  sendLog('successfully got dirHandle');
  setDirHandle(dirHandle);
}

async function getAllFileKeys(dirHandle: any): Promise<string[]> {
  const out: string[] = [];

  for await(const [key, _] of dirHandle.entries()) {
    if (key.endsWith('.png') || key.endsWith('.PNG')) {
      out.push(key);
    }
  }
  sendLog(`got ${out.length} file keys`);

  return out;
}

async function getFile(dirHandle: any, fileKey: string): Promise<File> {
  const fileHandle = await dirHandle.getFileHandle(fileKey);
  const file: File = await fileHandle.getFile();
  return file;
}

function sendLog(log: any) {
  fetch("/log", {
    method: 'POST',
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(log)
  })
}

type SAPScreenshot = {
  fileKey: string;
  numHearts: number;
  hasBandage: boolean;
  // set to true for files that arent actually a SAP screenshot
  invalid?: boolean;
};

function getCachedData(): { [key: string]: SAPScreenshot } {
  const screenshotmap = window.localStorage.getItem('sapscreenshots')
  if (!screenshotmap) { return {} }
  try {
    const obj = JSON.parse(screenshotmap);
    return obj;
  } catch (e) {
    return {};
  }
}

function App() {
  useEffect(() => {
    window.onerror = (errorMsg, _url, lineNumber) => {
      sendLog({ errorMsg, lineNumber });
      return false;
    }
  }, []);
  const [cachedData, setCachedDataState] = useState(getCachedData());
  const [wasm, setWasm] = useState<WebAssembly.WebAssemblyInstantiatedSource | null>(null);
  const [debugWasmResult, setDebugWasmResult] = useState('unknown wasm result...');
  const [dirHandle, setDirHandle] = useState(null);
  const [screenshots, setScreenshots] = useState<SAPScreenshot[]>([]);
  const [totalWins, setTotalWins] = useState(0);
  const [startDateValue, setStartDateValue] = useState(0);
  const appendCachedData = useCallback((obj: SAPScreenshot) => {
    setCachedDataState((prev) => {
      const newObj = { ...prev };
      newObj[obj.fileKey] = obj;
      try {
        const serialized = JSON.stringify(newObj);
        window.localStorage.setItem('sapscreenshots', serialized);
      } catch (e: any) {
        sendLog(`failed to serialize cached data: ${e.toString()}`);
      }
      return newObj;
    })
  }, [setCachedDataState]);
  useEffect(() => {
    if (!dirHandle || !wasm) { return }
    const doStuff = async () => {
      try {
        const fileKeys = await getAllFileKeys(dirHandle);
        sendLog('got file keys');
        const len = fileKeys.length;
        for (let i = 0; i < len; i += 1) {
          const fileKey = `${fileKeys[i]}`;
          const cachedObj = cachedData?.[fileKey];
          if (cachedObj) {
            if (!cachedObj.invalid) {
              setScreenshots((prev) => {
                const newPrev = [...prev];
                newPrev.push(cachedObj);
                return newPrev;
              });
              sendLog(`skipping processing ${fileKey} since its already cached`);
            }
            continue;
          }
          const match = fileKey.match(/_(\d*)-/);
          let date = 99999999;
          if (match) {
            const num = parseInt(match?.[1]);
            if (!Number.isNaN(num) && num > 0) {
              date = num;
            }
          }
          // skip dates that the user specified as being before their desired startDateValue
          if (date < startDateValue) {
            continue
          }
          const file = await getFile(dirHandle, fileKey);
          sendLog(`got file file ${file.name}. size: ${file.size}`);
          const ab = await file.arrayBuffer();
          // @ts-ignore
          const ptr = wasm.instance.exports.alloc(file.size);
          // @ts-ignore
          var mem = new Uint8Array(wasm.instance.exports.memory.buffer, ptr, file.size);
          // copy the content of the file into the memory buffer
          const abWindow = new Uint8Array(ab);
          mem.set(abWindow);

          // @ts-ignore
          let res: number = wasm.instance.exports.wasm_entrypoint(ptr, file.size);
          if (res == -1) {
            sendLog(`not a pets screenshot: ${fileKey}`);
            appendCachedData({ fileKey, numHearts: 0, hasBandage: false, invalid: true });
            continue;
          }

          const mask = 1 << 3;
          let numHearts = res;
          let hasBandage = false;
          if ((res & mask) != 0) {
            // has bandage, need to clear the bit
            numHearts = numHearts &= ~mask;
            hasBandage = true;
          }
          const sapscreenshot: SAPScreenshot = { fileKey, numHearts, hasBandage };
          setScreenshots((prev) => {
            const newPrev = [...prev];
            newPrev.push(sapscreenshot);
            return newPrev;
          });
          appendCachedData(sapscreenshot);
          setTotalWins((prev) => {
            return prev + 1;
          });
          sendLog(`${fileKey} : numHearts=${numHearts}, hasBandage=${hasBandage}`);
        }
        sendLog(`got all ${fileKeys.length} files`);

      } catch (e: any) {
        sendLog({ err: e.toString(), msg: 'failed to get all files' });
      }
      sendLog('after getFileBytes...');
    };
    doStuff();
  }, [dirHandle, wasm]);

  useEffect(() => {
    const doStuff = async () => {
      sendLog('loading wasm...');
      try {
        const memory = new WebAssembly.Memory({
          initial: 10,
          maximum: 200,
        });
        const instance = await WebAssembly.instantiateStreaming(fetch("wasm.wasm"), {js: { mem: memory }});
        // @ts-ignore
        instance.instance.exports.memory.grow(190);
        // @ts-ignore
        const myData = new Uint8Array(instance.instance.exports.memory.buffer, 0, 10);
        myData.set([2,1,1,1,1,1,1,1,1,1]);
        // @ts-ignore
        let res = instance.instance.exports.wasm_debug_sum(0, 10);
        setWasm(instance);
        setDebugWasmResult(`wasm returned ${res}`);
      } catch (e: any) {
        sendLog({ msg: 'failed to load wasm', err: e.toString()})
      }
    }
    if (!wasm) {
      doStuff();
    }
  }, [wasm, setWasm, setDebugWasmResult]);

  if (!wasm) {
    return <div>loading wasm...</div>
  }

  return (
    <>    
      <h2>{debugWasmResult}</h2>
      <button
        onClick={(e) => {
          e.preventDefault();
          getDir(setDirHandle);
        }}
      >
        pick directory
      </button>
      <h3>total wins: {totalWins}</h3>
      <label>start at date: {startDateValue}</label>
      <input
        disabled={Boolean(dirHandle)}
        type="text"
        value={startDateValue}
        onChange={(e) => {
          e.preventDefault();
          const num = parseInt(e.target.value);
          if (!Number.isNaN(num) && num > 0) {
            setStartDateValue(num);
          }
        }}
      />
      <ul>
        {screenshots.map(m => <ScreenshotItem key={m.fileKey} screenshot={m} />)}
      </ul>
    </>
  )
}

function ScreenshotItem({ screenshot }: { screenshot: SAPScreenshot}) {
  const bandageText = screenshot.hasBandage ? 'has bandage' : '';
  return (
    <li>
      {screenshot.fileKey} has {screenshot.numHearts} {bandageText}
    </li>
  )
}

export default App
