import {
	// IDataObject,
	IExecuteFunctions,
	INodeExecutionData,
	INodeType,
	INodeTypeDescription,
	NodeApiError,
	NodeConnectionType,
	NodeExecutionWithMetadata,
} from 'n8n-workflow';
import { execute_wasm, WasiExecution } from './addon';
import { isArgs, isEnv, isStdin } from './utils';
import path from 'node:path';

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
				displayName: 'WASM Executable',
				name: 'wasmExecutable',
				type: 'string',
				default: '',
				required: true,
				description: 'A wasm executable',
			},
			{
				displayName: 'Arguments',
				name: 'args',
				type: 'string',
				default: '[]',
				description: 'Arguments to pass to the wasm executable, as array',
			},
			{
				displayName: 'Environment Variables',
				name: 'env',
				type: 'string',
				default: '{}',
				description: 'Environment variables to set for the wasm executable',
			},
			{
				displayName: 'Stdin',
				name: 'stdin',
				type: 'string',
				default: '',
				description: 'Input to pass to the wasm executable via stdin',
			},
		],
	};

	async execute(
		this: IExecuteFunctions,
	): Promise<INodeExecutionData[][] | NodeExecutionWithMetadata[][] | null> {
		const items = this.getInputData();
		const basePath = process.env['N8N_WASM_DIRECTORY'] ?? __dirname;

		const returnData = [];

		// For each item, make an API call to create a contact
		for (let i = 0; i < items.length; i++) {
			const wasmExecutable = this.getNodeParameter('wasmExecutable', i) as string;
			const args = JSON.parse((this.getNodeParameter('args', i) ?? '[]') as string);
			const env = JSON.parse((this.getNodeParameter('env', i) ?? '{}') as string);
			const stdin = (this.getNodeParameter('stdin', i) ?? '') as string;

			// TODO: Support different forms of wasm executable sources
			// build out wasm modules lookup in directory maybe with a json manifest, etc.

			const wasmFile = path.join(basePath, wasmExecutable);

			if (!isArgs(args)) {
				throw new NodeApiError(this.getNode(), {
					message: `Invalid arguments: ${JSON.stringify(args)}`,
				});
			}

			if (!isEnv(env)) {
				throw new NodeApiError(this.getNode(), {
					message: `Invalid environment variables: ${JSON.stringify(env)}`,
				});
			}

			if (!isStdin(stdin)) {
				throw new NodeApiError(this.getNode(), {
					message: `Invalid stdin: ${JSON.stringify(stdin)}`,
				});
			}

			console.log(`WasmExecutable: ${wasmFile}`);
			const execution: WasiExecution = {
				program: wasmFile,
				args,
				env,
				stdin,
			};

			console.log(`Execution: ${JSON.stringify(execution)}`);

			console.log('Executing wasm');
			const start = performance.now();
			const result = execute_wasm(execution);
			const duration = performance.now() - start;

			console.log('Executed wasm', result, duration);

			const stdout = result.stdout?.split('\n')
				.map((line) => line.trim())
				.filter((line) => line) ?? [];

			const stderr = result.stderr?.split('\n')
				.map((line) => line.trim())
				.filter((line) => line) ?? [];

			returnData.push({
				...result,
				stdout,
				stderr,
				execution_time: duration,
			});
		}
		return [this.helpers.returnJsonArray(returnData)];
	}
}
