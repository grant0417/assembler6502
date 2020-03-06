        VAL = $01
        OLD = $FD
        NEW = $FE
        JMP RESET   ; Sets up the inital conditions
LOOP:   ADC NEW     ; Adds the last value
        LDX OLD     ; Moves the
        STX NEW
        STA OLD     ; Stores the new number
        BVS RESET   ; Resets if overflow flags is set
        JMP LOOP    ; Otherwise loops
RESET:  CLC
        LDA #VAL    ; Resets values to inital loop conditions
        STA NEW
        LDA #$00
        STA OLD
        JMP LOOP    ; Returns to loop