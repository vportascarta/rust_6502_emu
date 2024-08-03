; This program multiply by 10
LoadACImm      16

ArmLShfAC
StoreACAbs      $000b
ArmLShfAC
ArmLShfAC
ClrCarry
AddAbs          $000b
StoreACAbs      $000b

EmuSignal #FF