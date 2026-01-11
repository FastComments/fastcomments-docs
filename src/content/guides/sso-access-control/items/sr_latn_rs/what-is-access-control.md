Sa FastComments SSO Access Control, ponekad nazvanim RBAC, korisnicima se može ograničiti pristup samo određenim stranicama ili nitima komentara. Dodatno, korisnici mogu `@mention` jedni druge samo u istoj grupi.

## Detaljno

Možemo smestiti `Users` i opciono `Pages` u grupe.

Kada su `Users` smešteni u grupe, i `Limit Comments by SSO User Groups` je omogućen u Podešavanjima widgeta, tada će korisnici videti samo komentare od korisnika u njihovim istim grupama i moći će da `@mention` samo korisnike u istim grupama.

Dodatno, `Pages` se mogu smestiti u grupe, i tada korisnici mogu pristupati komentarima samo za stranice kojima imaju pristup.

Ovo nazivamo "Grupe na nivou korisnika" naspram "Grupe na nivou stranice".

Pa koja je prava za vas?

#### Koristite grupe na nivou korisnika ako...

- Želite da koristite istu vrednost `urlId` (URL stranice, ili ID članka), ali da ograničite komentare po grupi.
- Na primer, želite da imate grupe "Novi korisnik" i "Veteran korisnik", i da oni nikada ne vide komentare jedni drugih na istim stranicama.

#### Koristite grupe na nivou stranice ako...

- Vaše grupe imaju specifične stranice.
- Na primer, korisnici u grupi "Javne stranice" nikada ne bi smeli da vide članke iz grupe "Tajni članci".