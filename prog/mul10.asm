; This program multiply by 10
LoadACImm      16

ArmLShfAC
StoreACAbs      $200b
ArmLShfAC
ArmLShfAC
ClrCarry
AddAbs          $200b
StoreACAbs      $200b

Jam