---
Ker so prijavne povezave v bistvu gesla, varnost jemljemo zelo resno.

Vse prijavne povezave v našem sistemu potečejo po določenem času, obenem pa imamo mehanizme za odkrivanje
ugibanja prijavne povezave. Nekatere prijavne povezave so razdeljene na več gesel, in če je eno uganjeno,
ostalo bo razveljavljeno.

### Varnost v primerjavi z gesli

Pri večini sistemov, ki zahtevajo geslo, lahko uporabite mehanizem "Pozabljeno geslo"
če imate uporabnikov e-poštni naslov. To pomeni, da če imate dostop do uporabnikovega e-poštnega računa,
ni pomembno, ali sistem, ki je tarča napada, uporablja gesla ali čarobne povezave.

### Opozorila ob prijavi z novega IP naslova

Ko pride do prijave z naslova IP, ki še ni bil zabeležen za določen račun, FastComments pošlje varnostno opozorilo po e-pošti
s približno lokacijo in naslovom IP. To pomaga uporabnikom pri odkrivanju nepooblaščenega dostopa. Upoštevajte, da FastComments ne shranjuje
surovih naslovov IP — shranjena je le zamegljena oblika iz varnostnih razlogov.

### Varnost v primerjavi z MFA

Prijavne povezave so manj varne kot MFA. FastComments zdaj podpira dvofaktorsko avtentikacijo (2FA)
za administratorske račune za dodatno varnost. Ko je 2FA omogočen, je obvezna tudi pri uporabi prijavnih povezav.

---