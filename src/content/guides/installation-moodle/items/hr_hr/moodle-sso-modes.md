Dodatak podržava tri SSO načina za integraciju Moodle korisničkih računa s FastComments.

#### Sigurni SSO (Preporučeno)

Podaci o korisniku potpisuju se na strani poslužitelja koristeći HMAC-SHA256 s vašim API Secret. Ovo je najsigurnija opcija i preporučuje se za produkcijsko korištenje.

Kod Sigurnog SSO:

- Imena korisnika, e-mailovi i avatari prenose se sigurno u FastComments.
- Administratori Moodle stranice automatski se sinkroniziraju kao FastComments moderatori.
- Korisnici ne mogu preuzimati identitet drugih računa.
- Zahtijeva da je **API Secret** konfiguriran u postavkama dodatka.

#### Jednostavni SSO

Podaci o korisniku (ime, e-mail, avatar) šalju se na strani klijenta bez kriptografskog potpisa. To je lakše za postavljanje jer ne zahtijeva API Secret, ali je manje sigurno jer se podaci o korisniku ne provjeravaju na strani poslužitelja.

#### Nema integracije

Bez SSO integracije. Korisnici komentiraju anonimno ili se moraju osobno prijaviti u FastComments. Koristite ovo ako ne želite da Moodle korisnički računi budu povezani s komentarima.

---