const { readFileSync } =  require('node:fs')
const { resolve } =  require('node:path')

const buff = readFileSync(resolve(__dirname, 'fixtures/input.txt'))
const lines = buff.toString('utf8').split('\n')
const stackLines = lines.slice(0, 8)
const instructionLines = lines.slice(10, lines.length).filter(line => line.trim().length > 0)

const stacks = [ [], [], [], [], [], [], [], [], [] ]

for (const line of stackLines.reverse()) {
    stacks.forEach((stack, index) => {
        const pointer = (index * 4) + 1
        const crate = line.slice(pointer, pointer +1)

        if (crate.trim()) stack.push(crate)
    })
}

for (const instruction of instructionLines) {
    const matches = instruction.match(/move (\d+) from (\d+) to (\d+)/)
    const count = parseInt(matches[1])
    const from = parseInt(matches[2]) - 1
    const to = parseInt(matches[3]) - 1

    const toStack = stacks[to]
    const fromStack = stacks[from]

    const movedCrates = fromStack.splice(fromStack.length - count, count)

    toStack.push(...movedCrates)

}

console.log(stacks.reduce((acc, stack) => `${acc}${stack[stack.length - 1]}`, ''))
