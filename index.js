import init, { Emulator } from './static/wasm.js'

init().then(() => {
    const evaluator = new Emulator()
    evaluator.reset()
    evaluator.set_cpu_debug_callback(console.log)
    evaluator.set_memory_debug_callback(console.log)

    const input = document.getElementById('input')
    const loadButton = document.getElementById('load')
    const runButton = document.getElementById('run')
    const stepButton = document.getElementById('step')
    const resetButton = document.getElementById('reset')
    const output = document.getElementById('output')
    const memoryDump = document.getElementById('memorydump')

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

    stepButton.addEventListener('click', () => {
        try {
            evaluator.step()
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
})
