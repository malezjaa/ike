import {
    ByteLengthQueuingStrategy,
    CountQueuingStrategy,
    ReadableByteStreamController,
    ReadableStream,
    ReadableStreamBYOBReader,
    ReadableStreamBYOBRequest,
    ReadableStreamDefaultController,
    ReadableStreamDefaultReader,
    TransformStream,
    TransformStreamDefaultController,
    WritableStream,
    WritableStreamDefaultController,
    WritableStreamDefaultWriter,
} from "@std/streams";
import { clearTimeout, setTimeout } from "module:web/timeouts.js";
import { atob, btoa } from "module:web/base64.js";

const registerGlobal = (name, value) => {
    Object.defineProperty(globalThis, name, {
        value,
        writable: true,
        configurable: true,
    });
};

const registerIkeGlobal = (name, value) => {
    Object.defineProperty(globalThis.Ike, name, {
        value,
        writable: true,
        configurable: true,
    });
};

import { TextDecoder, TextEncoder } from "module:web/encoding.js";

const exports = {
    ReadableStream,
    ReadableStreamDefaultController,
    ReadableByteStreamController,
    ReadableStreamBYOBRequest,
    ReadableStreamDefaultReader,
    ReadableStreamBYOBReader,

    WritableStream,
    WritableStreamDefaultController,
    WritableStreamDefaultWriter,

    ByteLengthQueuingStrategy,
    CountQueuingStrategy,

    TransformStream,
    TransformStreamDefaultController,

    setTimeout,
    clearTimeout,

    atob,
    btoa,

    TextDecoder,
    TextEncoder,
};

for (const prop in exports) {
    if (Object.prototype.hasOwnProperty.call(exports, prop)) {
        registerGlobal(prop, exports[prop]);
    }
}

import {
    createDir,
    createDirSync,
    createFile,
    createFileSync,
    existsSync,
    readFile,
    readFileSync,
    readTextFile,
    readTextFileSync,
    remove,
    removeSync,
} from "module:fs/fs.js";

const ikeExports = {
    createDir,
    createDirSync,
    createFile,
    createFileSync,
    existsSync,
    readFile,
    readFileSync,
    readTextFile,
    readTextFileSync,
    remove,
    removeSync,
};

for (const prop in ikeExports) {
    if (Object.prototype.hasOwnProperty.call(ikeExports, prop)) {
        registerIkeGlobal(prop, ikeExports[prop]);
    }
}

globalThis.Ike.path = await import("@std/path");
