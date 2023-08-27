import init, { Emulator } from './static/wasm.js'

init().then(() => {
    const evaluator = new Emulator()
    evaluator.reset()

    const input = document.getElementById('input')
    const loadButton = document.getElementById('load')
    const runButton = document.getElementById('run')
    const resetButton = document.getElementById('reset')

    const output = document.getElementById('output')
    const memoryDump = document.getElementById('memorydump')

    const stepButton = document.getElementById('step')
    const clearButton = document.getElementById('clear')
    const debug_output = document.getElementById('debug_output')

    evaluator.set_cpu_debug_callback((msg) => {
        debug_output.value += `[CPU Debug]      : ${msg}\n`
        debug_output.scrollTop = debug_output.scrollHeight
    })

    evaluator.set_memory_debug_callback((msg) => {
        debug_output.value += `[Memory Debug]   : ${msg}\n`
        debug_output.scrollTop = debug_output.scrollHeight
    })

    evaluator.set_registers_debug_callback((msg) => {
        debug_output.value += `[Register Debug] : ${msg}\n`
        debug_output.scrollTop = debug_output.scrollHeight
    })

    input.value = `
LDA #$02
CMP #$01
BNE FOO
LDA #$01
STA $00
BRK

FOO:
    LDA #$01
    STA $01
    BRK
                `.trim()

    const update_output = () => {
        output.value = evaluator.cpu_status()

        memoryDump.value = ''
        memoryDump.value += evaluator.memory_hexdump(0x0000, 0x10)
        memoryDump.value += '\n'
        memoryDump.value += evaluator.memory_hexdump(0x8000, 0x8030)
    }

    update_output()

    loadButton.addEventListener('click', () => {
        try {
            evaluator.reset()

            let input = document.getElementById('input').value
            let assembled = evaluator.assemble(input)
            console.log(input, assembled)

            evaluator.load(assembled)

            update_output()
        } catch (e) {
            output.value = `Error: ${e}`
        }
    })

    runButton.addEventListener('click', () => {
        try {
            evaluator.execute()
            update_output()
        } catch (e) {
            output.value = `Error: ${e}`
        }
    })

    resetButton.addEventListener('click', () => {
        try {
            evaluator.reset()
            update_output()
        } catch (e) {
            output.value = `Error: ${e}`
        }
    })

    stepButton.addEventListener('click', () => {
        try {
            evaluator.step()
            update_output()
        } catch (e) {
            output.value = `Error: ${e}`
        }
    })

    clearButton.addEventListener('click', () => {
        debug_output.value = ''
    })
})
