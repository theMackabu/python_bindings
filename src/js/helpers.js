const formatChain = (string, format = false) =>
	JSON.stringify(string, null, format ? 2 : 0)
		.replace(/\\n/g, '\n')
		.replace(/\\'/g, "'")
		.replace(/\\"/g, '"')
		.replace(/\\&/g, '&')
		.replace(/\\r/g, '\r')
		.replace(/\\t/g, '\t')
		.replace(/\\b/g, '\b')
		.replace(/\\f/g, '\f');

const logWithoutObject = (...args) =>
	args.map((arg) => (typeof arg == 'object' ? formatChain(arg) : formatChain(arg.toString()).slice(1, arg.toString().length + 1))).join(' ');
