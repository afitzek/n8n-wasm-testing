
import {
    // IDataObject,
    IExecuteFunctions,
    INodeExecutionData,
    INodeType,
    INodeTypeDescription,
    NodeConnectionType,
    NodeExecutionWithMetadata,
} from 'n8n-workflow';

// import {
//     OptionsWithUri,
// } from 'request';

import { WASI } from 'node:wasi';
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { argv, env } from 'node:process';

// const loadWasmer = (async () => {
//     const {
//         init, Wasmer
//     } = await import("@wasmer/sdk");

//     try {
//         await init();
//     } catch(e) {
//         console.error(`Failed to run init!`, e);
//     }

//     return Wasmer;
// })()

export class WasmExecutor implements INodeType {
    description: INodeTypeDescription = {
        displayName: 'WasmExecutor',
        name: 'wasmExecutor',
        icon: 'file:wasmExecutor.svg',
        group: ['transform'],
        version: 1,
        description: 'Execute a wasm binary',
        defaults: {
            name: 'WasmExecutor',
        },
        inputs: [NodeConnectionType.Main],
        outputs: [NodeConnectionType.Main],
        properties: [
            {
                displayName: 'WasmBinary',
                name: 'wasmBinary',
                type: 'string',
                default: '',
                required: true,
                description: 'A wasm binary',
            }
        ]
    }

    async execute(this: IExecuteFunctions): Promise<INodeExecutionData[][] | NodeExecutionWithMetadata[][] | null> {
        const items = this.getInputData();
        
        const returnData = [];
        const wasmBinary = this.getNodeParameter('wasmBinary', 0) as string;
        // const Wasmer = await loadWasmer;

        // const pkg = await Wasmer.fromRegistry(wasmBinary);

        const wasi = new WASI({
            version: 'preview1',
            args: argv,
            env,
            preopens: {
                '/local': '/tmp',
            },
        });
        const wasm = await WebAssembly.compile(
            await readFile(join(__dirname, wasmBinary)),
        );
        // For each item, make an API call to create a contact
        for (let i = 0; i < items.length; i++) {
            // const instance = await pkg.entrypoint!.run({
            //     args: ["-c", "print('Hello, World!')"]
            // });
            // const {
            //     code
            // } = await instance?.wait();
            const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject() as WebAssembly.Imports);
            const code = await wasi.start(instance);
            returnData.push({
                code
            });
        }
        return [this.helpers.returnJsonArray(returnData)];
    }
}