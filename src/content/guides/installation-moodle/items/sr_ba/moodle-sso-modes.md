Dodatak podržava tri SSO moda za integraciju Moodle korisničkih naloga sa FastComments.

#### Sigurni SSO (Preporučeno)

Podaci o korisniku se potpisuju na serverskoj strani koristeći HMAC-SHA256 sa vašim API Secret. Ovo je najsigurnija opcija i preporučuje se za produkcijsko korištenje.

Sa sigurnim SSO:

- Imena korisnika, emailovi i avatari se sigurno prenose u FastComments.
- Administratori Moodle sajta se automatski sinhronizuju kao FastComments moderatori.
- Korisnici ne mogu lažno predstavljati druge naloge.
- Zahteva da **API Secret** bude konfigurisan u podešavanjima dodatka.

#### Jednostavni SSO

Podaci o korisniku (ime, email, avatar) se šalju sa klijentske strane bez kriptografskog potpisa. Ovo je lakše za postavljanje jer ne zahteva API Secret, ali je manje sigurno jer se podaci o korisniku ne proveravaju na serverskoj strani.

#### Bez SSO

Bez SSO integracije. Korisnici komentarišu anonimno ili se moraju odvojeno prijaviti na FastComments. Koristite ovo ako ne želite da Moodle korisnički nalozi budu povezani sa komentarima.