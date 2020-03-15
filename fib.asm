        VAL = $01
        OLD = $FD
        NEW = $FE
        JMP RESET   ; Sets up the inital conditions
LOOP:   ADC OLD     ; Adds the old value to last value
        BCS RESET   ; Resets if carry flags is set
        LDX NEW     ; Moves the last value to old
        STX OLD
        STA NEW     ; Stores the new number in new
        JMP LOOP    ; Loop
RESET:  CLC
        LDA #VAL    ; Resets values to inital loop conditions
        STA OLD
        LDA #$00
        STA NEW
        JMP LOOP    ; Returns to loop