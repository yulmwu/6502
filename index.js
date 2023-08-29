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

    const range_start = document.getElementById('range_start')
    const range_end = document.getElementById('range_end')
    const dumpButton = document.getElementById('dump')

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

        const start = parseInt(range_start.value, 16)
        const end = parseInt(range_end.value, 16)
        console.log(start, end)
        if (isNaN(start) || isNaN(end)) {
            memoryDump.value = 'Invalid range'
            return
        }
        memoryDump.value = evaluator.memory_hexdump(start, end)
    }

    update_output()

    loadButton.addEventListener('click', () => {
        try {
            evaluator.reset()

            let input = document.getElementById('input').value
            console.log(input)
            let assembled = evaluator.assemble(input)

            if (assembled.kind() === 1) {
                update_output()
                output.value = `Error: ${assembled.error()}`
            } else {
                evaluator.load(assembled.value())
                update_output()
            }
        } catch (e) {
            output.value = e
        }
    })

    runButton.addEventListener('click', () => {
        try {
            evaluator.execute()
            update_output()
        } catch (e) {
            output.value = e
        }
    })

    resetButton.addEventListener('click', () => {
        try {
            evaluator.reset()
            update_output()
        } catch (e) {
            output.value = e
        }
    })

    dumpButton.addEventListener('click', () => {
        try {
            update_output()
        } catch (e) {
            output.value = e
        }
    })

    stepButton.addEventListener('click', () => {
        try {
            evaluator.step()
            update_output()
        } catch (e) {
            output.value = e
        }
    })

    clearButton.addEventListener('click', () => {
        debug_output.value = ''
    })
})

document.getElementById('presets').addEventListener('change', (e) => {
    const split = e.target.value.split(',')
    document.getElementById('range_start').value = split[0]
    document.getElementById('range_end').value = split[1]
})
