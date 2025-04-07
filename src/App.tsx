import { useEffect, useState } from 'react'
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
  sendLog(`requesting filekey: ${fileKey}`);
  const fileHandle = await dirHandle.getFileHandle(fileKey);
  sendLog(`got file handle for ${fileKey}`);
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

function App() {
  useEffect(() => {
    window.onerror = (errorMsg, _url, lineNumber) => {
      sendLog({ errorMsg, lineNumber });
      return false;
    }
  }, []);
  const [wasm, setWasm] = useState<WebAssembly.WebAssemblyInstantiatedSource | null>(null);
  const [debugWasmResult, setDebugWasmResult] = useState('unknown wasm result...');
  const [dirHandle, setDirHandle] = useState(null);
  useEffect(() => {
    if (!dirHandle || !wasm) { return }
    const doStuff = async () => {
      try {
        const fileKeys = await getAllFileKeys(dirHandle);
        sendLog('got file keys');
        // const len = fileKeys.length;
        const len = 2;
        for (let i = 0; i < len; i += 1) {
          const file = await getFile(dirHandle, fileKeys[i]);
          sendLog(`got file file ${file.name}. size: ${file.size}`);
          const ab = await file.arrayBuffer();
          sendLog(`got arrayBuffer: ${ab.byteLength}`);


          // @ts-ignore
          const wasmMemory = new Uint8Array(wasm.instance.exports.memory.buffer);
          sendLog(`got wasmMemory: ${wasmMemory.byteOffset}, ${wasmMemory.byteLength}`);

          const data = new Uint8Array(ab); // wrap ArrayBuffer
          const ptr = 0; // assume offset 0 or use a proper allocator in real code
          sendLog(`got new data array: ${data.byteOffset}, ${data.byteLength}`);
      
          // Copy your buffer into wasm memory
          wasmMemory.set(data, ptr);
          sendLog(`successfully set wasmmemory`);

          const numBytesToSum = ab.byteLength;
          // @ts-ignore
          let res = wasm.instance.exports.wasm_entrypoint(0, numBytesToSum);
          sendLog(`got wasm result for summing first ${numBytesToSum}bytes! ${res}`);
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
          initial: 200,
        });
        const instance = await WebAssembly.instantiateStreaming(fetch("wasm.wasm"), {js: { mem: memory }});
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
    </>
  )
}

export default App
