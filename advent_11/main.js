const fs = require('fs');

const data = fs.readFileSync('input.txt', 'utf8');
const monkeysParagraph = data.split(/\n\n/); // Splits the file content by new line



const monkeys = monkeysParagraph.map((m, i) => {
	var monkey = {}
	monkey.id = i
	monkey.thrown = 0
	const lines = m.split(/\n/)	

	monkey.starting = lines[1].replace('Starting items: ', '').replace(/\s+/g,'').split(',').map(n => BigInt(n))

	const op = lines[2].replace('Operation: new = ', '').trim().split(/\s+/g)
	monkey.a = bigIntOrNull(op[0])
	monkey.b = bigIntOrNull(op[2])

	if (op[1] === '+') monkey.op = add
	if (op[1] === '*') monkey.op = multiply

	monkey.test = BigInt(lines[3].replace('Test: divisible by', '' ).trim())
	monkey.testOk = BigInt(lines[4].replace('If true: throw to monkey', '').trim())
	monkey.testKo = BigInt(lines[5].replace('If false: throw to monkey', '').trim())

	return monkey
})

for (var i = 0; i<20; i++) {
	monkeys.forEach((monkey) => {
		monkey.starting.forEach((obj) => {
			const op1 = monkey.a === null? obj : monkey.a
			const op2 = monkey.b === null? obj : monkey.b
			const worry = monkey.op(op1,op2) / BigInt(3)

			const throwTo = worry % monkey.test === 0 ? monkey.testOk : monkey.testKo

			monkeys[throwTo].starting.push(worry)
			monkey.thrown++
		})
		monkey.starting = []
		console.log('round',i,'monkey',monkey.id,'thrown', monkey.thrown)
	})
}


monkeys.sort((ma,mb) => mb.thrown - ma.thrown)

console.log(monkeys[0].thrown * monkeys[1].thrown)

function add(a,b){
	return a + b
}

function multiply(a,b){
	return a*b
}

function bigIntOrNull(str) {
	try {
		return BigInt(str)
	}catch (e) {
		return null
	}
}
