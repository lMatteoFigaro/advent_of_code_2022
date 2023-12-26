const fs = require('fs');

const data = fs.readFileSync('input.txt', 'utf8');
const lines = data.split(/\r?\n/); // Splits the file content by new line
lines.pop()

var x = 1
lines.unshift('noop')
const val = lines
.map((line) => {
	const words = line.split(' ')
	const op = words[0]

	if (op == 'noop') return [0]
	return [0, parseInt(words[1])]
})
.flat()
.map((val, i) => {
	x += val
	return x 
})
.forEach((x,i) => { 
	const cycle = i + 1
	const pos = i % 40
	if ( pos >= x - 1 && pos <= x + 1) {
		// console.log('cycle is', cycle, 'x is', x, 'drawing #')
		process.stdout.write('#')
	} else {
		// console.log('cycle is', cycle, 'x is', x, 'drawing .')
		process.stdout.write('.')
	}

	if (cycle % 40 === 0 ) {
		console.log('')
	}
})

