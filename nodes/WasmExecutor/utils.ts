export function isArgs(obj: unknown): obj is Array<string> {
	return Array.isArray(obj) && obj.every((item) => typeof item === 'string');
}

export function isEnv(obj: unknown): obj is Record<string, string> {
	return typeof obj === 'object' && obj !== null && !Array.isArray(obj) && Object.entries(obj).every(([key, value]) => typeof key === 'string' && typeof value === 'string');
}

export function isStdin(obj: unknown): obj is string {
	return typeof obj === 'string';
}
