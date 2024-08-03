; This is a test program
LoadACImm      $A5
AddImm         16
StoreACZp      %00100000
NoOp

LoadACImm      %10101011
AddImm         %01010101
BranchZero     $05
JumpAbs 0
NoOp

Jam