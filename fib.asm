        VAL = $01
        JMP RESET   ; Sets up the inital conditions
LOOP    ADC $00     ; Adds the last value
        LDX $01     ; Moves the 
        STX $00
        STA $01     ; Stores the new number
        BVS RESET   ; Resets if overflow flags is set
        JMP LOOP    ; Otherwise loops
RESET   CLV
        LDA VAL   ; Resets values to inital loop conditions
        STA $00
        LDA #$00
        STA $01
        JMP LOOP   ; Returns to loop