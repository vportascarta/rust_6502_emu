
; This is a test program
LoadAccImm   #A5
StoreAccZp   %11111111
NoOp
;JumpAbs 0
NoOp
EmuSignal #FF