#### Spominjanje korisnika iz drugih grupa

Ako dva korisnika pripadaju dvema različitim skupovima grupa, i nema preklapanja, neće moći međusobno da `@mention`.

Ako korisnik ručno otkuca `@mention` i pošalje komentar, to će ostati običan tekst. Drugi korisnik neće biti označen.

#### Održavanje grupa

`Groups` se definišu korišćenjem API resursa `Pages` i `SSOUsers`, redom.

API `Pages` može biti pozvan da definiše skup grupa kojima je dozvoljen pristup stranici. Podrazumevano, sve grupe, kao i korisnici koji ne pripadaju nijednoj grupi, imaju pristup.

Slično, API `SSOUsers` može biti pozvan da definiše grupe povezane sa svakim korisnikom.

Za oba resursa ne postoje ograničenja u pogledu kada grupe mogu biti postavljene ili ažurirane.

Ako je cilj samo da se ograniči mogućnost da se korisnici međusobno koriste `@mention`, onda se `Pages` ne moraju uzimati u obzir.

##### Napomena!

Definisanje i ažuriranje SSO korisničkih grupa ne zahteva korišćenje API-ja, i umesto toga može biti ažurirano automatski definisanjem group ids u SSO payload-u koji se prosleđuje comment widget-u. Međutim, za velike liste grupa, ovo se ne preporučuje jer bi korisnik morao da pošalje ovaj payload pri svakom učitavanju stranice.