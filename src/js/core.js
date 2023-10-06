const { core } = Deno;
const { ops } = core;

globalThis.console = {
	log: (...args) => core.print(logWithoutObject(...args) + '\n', false),
	error: (...args) => ops.print(logWithoutObject(...args) + '\n', true),
};

globalThis.runtime = {
	readFile: (path) => ops.op_read_file(path),
	writeFile: (path, contents) => ops.op_write_file(path, contents),
	removeFile: (path) => ops.op_remove_file(path),
	fetch: async (url) => core.opAsync('op_fetch', url),
};

globalThis.core = core;
globalThis.setTimeout = (callback, delay) => core.opAsync('op_set_timeout', delay).then(callback);
