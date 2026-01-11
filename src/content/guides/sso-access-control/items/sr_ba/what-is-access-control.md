Sa FastComments SSO kontrolom pristupa, ponekad nazivanom RBAC, korisnicima se može ograničiti pristup samo određenim stranicama ili nitima komentara. Također,
korisnici mogu `@mention` jedni druge samo u istoj grupi.

## Detaljno

Možemo smjestiti `Users` i opcionalno `Pages` u grupe.

Kada su `Users` smješteni u grupe, i kada je `Limit Comments by SSO User Groups` omogućen u Postavkama widgeta, onda korisnici
će vidjeti samo komentare od korisnika iz istih grupa i moći će `@mention` samo korisnike iz istih grupa.

Također, `Pages` se mogu smjestiti u grupe, i tada korisnici mogu pristupiti komentarima samo za stranice kojima imaju pristup.

Ovo nazivamo "User-Level" grupama nasuprot "Page-Level" grupama.

Dakle, koja je prava za vas?

#### Koristite User-Level grupe ako...

- Želite koristiti istu vrijednost `urlId` (URL stranice, ili ID članka), ali ograničiti komentare po grupi.
- Na primjer, želite imati grupe "Novi korisnik" i "Veteran korisnik", i one nikada ne bi trebale vidjeti komentare jedna druge na istim stranicama.

#### Koristite Page-Level grupe ako...

- Vaše grupe imaju specifične stranice.
- Na primjer, korisnici u grupi "Javne stranice" nikada ne bi trebali vidjeti članke iz grupe "Vrlo tajno".

---