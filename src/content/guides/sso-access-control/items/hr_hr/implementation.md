#### Spominjanje korisnika u drugim grupama

Ako dva korisnika pripadaju dvjema različitim skupinama grupa, i nema presjeka, neće moći `@mention`ati jedno drugo.

Ako korisnik ručno upiše `@mention` i pošalje svoj komentar, to će ostati običan tekst. Drugi korisnik neće biti označen.

#### Održavanje grupa

`Groups` se definiraju pomoću API resursa `Pages` i `SSOUsers`, redom.

API `Pages` može se pozvati kako bi se definirao skup grupa kojima je dopušten pristup stranici. Po zadanim postavkama sve grupe i korisnici koji ne pripadaju niti jednoj grupi imaju pristup.

Slično tome, API `SSOUsers` može se pozvati za definiranje grupa povezanih sa svakim korisnikom.

Za oba resursa ne postoje ograničenja kada se grupe mogu postaviti ili ažurirati.

Ako je jedini cilj ograničiti da se korisnici međusobno `@mention`aju, tada se `Pages` ne moraju uzimati u obzir.

##### Napomena!

Definiranje i ažuriranje SSO korisničkih grupa ne zahtijeva korištenje API-ja, već se može automatski ažurirati definiranjem group ids u SSO payloadu koji se prosljeđuje comment widgetu. Međutim, za velike popise grupa to se ne preporučuje jer bi korisnik morao poslati ovaj payload pri svakom učitavanju stranice.