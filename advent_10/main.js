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
	console.log('x', x, 'val',val,'cycle', i+1)
	return x 
})
.map((val,i) =>{
	const cycle = i +1
	if ((cycle-20) % 40 !== 0) return 0
	const cycleVal = val * cycle
	console.log('cycle', cycle, 'value', val, 'cycleVal', cycleVal)
	return cycleVal
})
.reduce((acc,val,i) => {
	return acc + val 
})

console.log(val)
