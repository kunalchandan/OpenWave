# OpenWave

Basic UI simulation of uSynth. Eventually will become firmware of uSynth.

## Stack

```mermaid
stateDiagram-v2
    state Inputs {
        state Dials {
            Dial1 : Dial x4 + Push Button
        }
        state Keyboard {
            Buttons : Buttons x24
        }
        Mic
        AudioJackIn : 3.5mm Audio IN
    }
    state Processing {
        MCU : Teensy 4.1 MCU
        FilterBank : BPF Variable Filter
        state Analog {
            ADC
            DAC
        }
    }
    state Outputs {
        Display
        AudioJackOut : 3.5mm Audio OUT
        USBC
    }
    %% Inputs --> Processing
    %% Processing --> Outputs
    MIDI : MIDI Output
    AudioJackIn --> ADC

    Keyboard --> MCU : PISO (Serial)
    Dials --> MCU : Interrupts
    MCU --> MIDI
    MCU --> DAC : I2C?

    DAC --> AudioJackOut
    DAC --> FilterBank
    FilterBank --> ADC

    Mic --> ADC
    ADC --> MCU : I2C?

    MCU --> Display
    MIDI --> USBC : Is this possible?
    USBC --> MCU : Power
    USBC --> MCU : Data Input (PC)
```

---


```mermaid
block-beta
    columns 4
    DisplayUI("Display UI")
    HardwareKeyboard("Keyboard Array")
    HardwareButtons("Misc. Buttons")
    HardwareKnobs("Hardware Knobs")

    Display("LCD/OLED Display")
    PISORegisters:3

    SlintLibrary("SlintUI Library")
    RTICS:3
    Rust:4
    Teensy:4
```


## Requirement Diagram

```mermaid
requirementDiagram
    requirement UI {
        id: 1
        Text: UI of the Synthesizer
        Risk: Low
        verifymethod: Demonstration
    }

    requirement RTOS {
        id: 2
        Text: RTOS of the Synth on the Teensy board
        Risk: Low
        verifymethod: Demonstration
    }

    element SlintLibrary {
        type: library
        docRef: crates.io/crates/slint
    }

    element RTICLibrary {
        type: library
        docRef: rtic.rs/2/book/en/
    }

    RTICLibrary - satisfies -> RTOS
    SlintLibrary - satisfies -> UI



```
