const fs = require('fs');

const data = fs.readFileSync('input.txt', 'utf8');
const lines = data.split(/\r?\n/); // Splits the file content by new line

var head = { x: 0, y:0 }
var tails = [{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 },
{ x: 0, y:0 }
]
lines.pop()
const pos = []
lines.map((line) => {
	const words = line.split(' ')
	const direction = words[0]
	const repeat = words[1];

	var moves = []

	for (var i = 0;i < repeat; i++ ){
		moves.push(direction)
	}
	return moves
})
.flat()
.map((direction) => {
	moveHead(direction,head)
	var previous = head

	for (const tail of tails){
		moveTail(previous,tail)
		previous = tail
	}
	console.log('head', head)
	console.log('tails', tails)
	const last = tails[tails.length - 1]

	return {x:last.x,y:last.y}
}).forEach((item,i,self) =>{
	if (pos.some(val => val.x === item.x && val.y === item.y)) return
	pos.push(item)
})

console.log(pos.length)

function moveHead(direction, head){
	switch(direction){
		case 'R':
			head.x++
			break
		case 'U':
			head.y++
			break
		case 'L': 
			head.x--
			break
		case 'D': 
			head.y--
			break
	}
}

function moveTail(head,tail){
	const xDistance = head.x - tail.x
	const yDistance = head.y - tail.y 
	const distance = Math.sqrt( xDistance * xDistance + yDistance * yDistance)
	if (distance < 1.5) return
	if ( head.x == tail.x) {
		if (tail.y > head.y) {tail.y--; return}
		if (tail.y  < head.y) {tail.y++; return}
	}

	if (head.y == tail.y) {
		if ( tail.x > head.x) { tail.x--; return }
		if ( tail.x < head.x) { tail.x++; return }
	}

	if (head.x > tail.x){
		if ( head.y > tail.y) { tail.x++; tail.y++; return }
		if ( head.y < tail.y) { tail.x++; tail.y--; return }
	}

	if (head.x < tail.x){
		if(head.y > tail.y ) { tail.x--; tail.y++; return}
		if (head.y < tail.y) {tail.x--; tail.y--; return}
	}
}
