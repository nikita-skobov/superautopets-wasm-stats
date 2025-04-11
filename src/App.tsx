import { useCallback, useEffect, useMemo, useState } from 'react'
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

function sendLog(log: any, alsoConsoleLog?: boolean) {
  if (alsoConsoleLog) {
    console.log(log);
  }
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
  turnCount: number;
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
          let res: BigInt = BigInt(wasm.instance.exports.wasm_entrypoint(ptr, file.size));
          if (res === -1n) {
            sendLog(`not a pets screenshot: ${fileKey}`);
            appendCachedData({ fileKey, numHearts: 0, hasBandage: false, invalid: true, turnCount: 0 });
            continue;
          }

          const mask = BigInt(1 << 3);
          // @ts-ignore
          let numHearts = res & 15n; // get only the 4 LSB bits
          let hasBandage = false;
          // @ts-ignore
          if ((res & mask) != 0) {
            // has bandage, need to clear the bit
            numHearts = numHearts &= ~mask;
            hasBandage = true;
          }
          // @ts-ignore
          let turnCount = res >> 4n;
          let base = 10n;
          // @ts-ignore
          if ((turnCount & 16n) != 0) {
            base = 20n;
            turnCount -= 10n;
          }
          //                 remove the 0b10000 bit
          // @ts-ignore
          turnCount = base + (turnCount & 15n);
          const sapscreenshot: SAPScreenshot = { fileKey, numHearts: Number(numHearts), hasBandage, turnCount: Number(turnCount) };
          setScreenshots((prev) => {
            const newPrev = [...prev];
            newPrev.push(sapscreenshot);
            return newPrev;
          });
          appendCachedData(sapscreenshot);
          sendLog(`${fileKey} : numHearts=${numHearts}, hasBandage=${hasBandage}`);
        }
        sendLog(`got all ${fileKeys.length} files`);

      } catch (e: any) {
        console.error(e);
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

  const totalWins = useMemo(() => {
    return screenshots.filter((s) => !s.invalid).length
  }, [screenshots]);

  const getDay = (dayOfWeek: number, datestring: string): boolean => {
    const match = datestring.match(/_(\d*)-/);
    let dateStr = '19900101';
    if (match) {
      dateStr = match?.[1] ?? '19900101';
    }

    const year = parseInt(dateStr.slice(0, 4), 10);
    const month = parseInt(dateStr.slice(4, 6), 10) - 1; // JS Date months are 0-based
    const day = parseInt(dateStr.slice(6, 8), 10);
  
    const date = new Date(year, month, day);
    
    // JS Date.getDay(): 0 = Sunday, 1 = Monday, ..., 6 = Saturday
    // Shift it so Monday = 0, ..., Sunday = 6
    const jsDay = date.getDay();
    const shiftedDay = (jsDay + 6) % 7;
    return shiftedDay === dayOfWeek;
  };

  const dayStats = useMemo(() => {
    const monday = screenshots.filter(s => !s.invalid).map(s => getDay(0, s.fileKey)).filter(Boolean).length;
    const tuesday = screenshots.filter(s => !s.invalid).map(s => getDay(1, s.fileKey)).filter(Boolean).length;
    const wednesday = screenshots.filter(s => !s.invalid).map(s => getDay(2, s.fileKey)).filter(Boolean).length;
    const thursday = screenshots.filter(s => !s.invalid).map(s => getDay(3, s.fileKey)).filter(Boolean).length;
    const friday = screenshots.filter(s => !s.invalid).map(s => getDay(4, s.fileKey)).filter(Boolean).length;
    const saturday = screenshots.filter(s => !s.invalid).map(s => getDay(5, s.fileKey)).filter(Boolean).length;
    const sunday = screenshots.filter(s => !s.invalid).map(s => getDay(6, s.fileKey)).filter(Boolean).length;
    return {
      monday,
      tuesday,
      wednesday,
      thursday,
      friday,
      saturday,
      sunday
    }
  }, [screenshots]);

  const turnStats = useMemo(() => {
    const turnCountMap: { [key: number]: number } = {};
    for (let i = 0; i < screenshots.length; i += 1) {
      const screenshot = screenshots[i];
      if (screenshot.invalid) { continue }
      const turnCount = screenshot.turnCount;
      if (turnCountMap.hasOwnProperty(turnCount)) {
        turnCountMap[turnCount] += 1;
      } else {
        turnCountMap[turnCount] = 1;
      }
    }
    const keys = Object.keys(turnCountMap);
    return keys.map(k => {
      const key = parseInt(k);
      return [key, turnCountMap[key]];
    }).sort((a, b) => {
      return a[0] - b[0]
    })
  }, [screenshots]);
  const winsWithBandage = useMemo(() => {
    const wins = screenshots.filter(x => !x.invalid);
    let winsWithB = 0;
    for (let i = 0; i < wins.length; i += 1) {
      if (wins[i].hasBandage) {
        winsWithB += 1;
      }
    }
    return winsWithB;
  }, [screenshots]);
  const winsWithoutBandage = useMemo(() => totalWins - winsWithBandage, [totalWins, winsWithBandage]);

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
      <h4>wins without bandage: {winsWithoutBandage}</h4>
      <h4>wins with bandage: {winsWithBandage}</h4>
      <ol>
        <li>wins on monday: {dayStats.monday} </li>
        <li>wins on tuesday: {dayStats.tuesday} </li>
        <li>wins on wednesday: {dayStats.wednesday} </li>
        <li>wins on thursday: {dayStats.thursday} </li>
        <li>wins on friday: {dayStats.friday} </li>
        <li>wins on saturday: {dayStats.saturday} </li>
        <li>wins on sunday: {dayStats.sunday} </li>
      </ol>
      <ol>
        {turnStats.map((val) => {
          const [turnCount, num] = val;
          return <li>Wins on turn {turnCount}: {num}</li>
        })}
      </ol>
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
      {screenshot.fileKey} won @turn={screenshot.turnCount}. has {screenshot.numHearts} {bandageText}
    </li>
  )
}

export default App
