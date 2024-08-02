
; This is a test program
LoadAccImm      #A5
AddCImm         16
StoreAccZp      %00100000
NoOp
LoadAccImm      %10101011
AddCImm         %01010101
BranchZ         #05
JumpAbs 0
NoOp
EmuSignal #FF