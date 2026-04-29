Orodje Ban je najpomembnejše dejanje, ki ga lahko agent sproži. Prepove uporabnika iz vaše skupnosti za določeno obdobje in ponuja nekaj možnosti.

### Kaj počne

Agent izbere eno od šestih trajanj:

- Ena ura
- En dan
- En teden
- En mesec
- Šest mesecev
- Ena leto

Prav tako izbere med **vidno prepovedjo** (uporabnik vidi jasen prikaz prepovedi in se lahko pritoži) in **senčno prepovedjo** (uporabnik lahko še naprej objavlja, vendar je njegova vsebina skrita drugim uporabnikom). Navodila platforme agentu priporočajo uporabo vidnih prepovedi za prve ali mejne primere, in senčnih prepovedi za očitne zlonamerne ponavljajoče kršilce.

### Dve uničujoči podmožnosti

Dve dodatni možnosti sta privzeto **skriti agentu**. Če želite omogočiti katero koli, potrdite ustrezno polje v razdelku **Ban options** na obrazcu za urejanje agenta:

- **Allow deleting all of the user's comments.** Ko je omogočeno, se agent lahko odloči tudi za izbris vseh komentarjev, ki jih je prepovedani uporabnik kdaj koli objavil v vašem tenant-u. Rezervirajte za očiten spam, doxxing ali usklajeno zlorabo, kjer obstoječa vsebina nima vrednosti. **Uničujoče in nepreklicno.**
- **Allow banning by IP.** Ko je omogočeno, se agent lahko odloči tudi za prepoved IP-ja, s katerega je bil komentar objavljen. Uporabno proti izogibanju prepovedi z alternativnimi računi. **Izogibajte se za deljene IP-je** (podjetniški, šolski, mobilni operaterji) - nedolžni uporabniki na isti mreži bodo blokirani.

Platforma to tudi omeji na strežniški strani: celo če agent uide nadzoru in poskusi uporabiti možnost, je zahteva zavrnjena, razen če ste jo omogočili.

### Politika eskalacije

Pred prepovedjo platforma agentu ukaže:

1. Preišče [spomin agenta](#agent-memory-system) po prejšnjih opozorilih ali zapiskih o uporabniku.
2. Pri prvih prekrških naj raje [opozori uporabnika](#tool-warn-user) kot ga prepove.
3. Korak z opozorilom preskoči le pri očitno resnih primerih (nezakonita vsebina, doxxing, usklajen spam) — in v svoji utemeljitvi pojasni zakaj.

Ta politika je v navodilih agenta, ne trda strežniška pravila, zato je močno priporočljivo, da za prepovedi zahtevate odobritev.

### Regija EU: potrebna človeška odobritev

V regiji EU je to orodje zaradi 17. člena Uredbe o digitalnih storitvah zaklenjeno za odobritev. Vsaka prepoved, ki jo sproži kateri koli agent v tenant-u v regiji EU, pristane v [predalu za odobritve](#approval-workflow) za človeški pregled. Glej [Skladnost z 17. členom DSA EU](#eu-dsa-compliance).

### Priporočila

- Zahtevajte odobritev povsod vsaj v prvem mesecu.
- Vedno omejite možnost **delete-all-comments**, če jo omogočite - je nepovratna.
- Premislite o omejitvi možnosti **IP ban** tudi potem, ko agent pridobi zaupanje - strošek prepovedi IP na deljeni mreži se ne pokaže v zgodovini izvajanja agenta.

### Oglejte si tudi

- [Prepovedovanje uporabnikov](/guide-moderation.html#banning-users) in [Prepovedovanje uporabnikov z nadomestnimi znaki](/guide-moderation.html#banning-users-wildcards) v priročniku za moderacijo za informacije o tem, kako prepovedi delujejo po celotni platformi.
- [Opozori uporabnika](#tool-warn-user) - blažja stopnja eskalacije.