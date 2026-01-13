With FastComments SSO kontrolom pristupa, ponekad nazvanom RBAC, korisnicima se može ograničiti pristup samo određenim stranicama ili nitima komentara. Osim toga,
korisnici mogu `@mention` jedni druge samo u istoj grupi.

## Detaljno

Možemo smjestiti `Users` i po potrebi `Pages` u grupe.

Kada su `Users` smješteni u grupe, i `Limit Comments by SSO User Groups` je omogućen u Widget Settings, tada korisnici
će vidjeti samo komentare od korisnika iz svojih istih grupa i moći će `@mention` samo korisnike iz istih grupa.

Dodatno, `Pages` se mogu smjestiti u grupe, i tada korisnici mogu pristupiti komentarima samo za stranice kojima imaju pristup.

To zovemo "User-Level" groups versus "Page-Level" groups.

Dakle, koja je prava za vas?

#### Koristite grupe na razini korisnika ako...

- Želite koristiti istu `urlId` vrijednost (URL stranice ili ID članka), ali ograničiti komentare po grupi.
- Na primjer, želite imati grupe "New User" i "Veteran User", i one nikada ne bi trebale vidjeti međusobne komentare na istim stranicama.

#### Koristite grupe na razini stranice ako...

- Vaše grupe imaju određene stranice.
- Na primjer, korisnici u grupi "Public Pages" nikada ne bi smjeli pregledavati članke iz grupe "Top Secret".

---