Plugin podržava tri SSO režima za integraciju Moodle korisničkih naloga sa FastComments.

#### Sigurni SSO (Preporučeno)

Podaci o korisniku se potpisuju na serverskoj strani koristeći HMAC-SHA256 sa vašim **API Secret**. Ovo je najsigurnija opcija i preporučuje se za proizvodnu upotrebu.

Sa Sigurnim SSO:

- Korisnička imena, email adrese i avatari se bezbedno prosleđuju FastComments-u.
- Administratori Moodle sajta se automatski sinhronizuju kao moderatori na FastComments.
- Korisnici ne mogu da lažno predstavljaju druge naloge.
- Zahteva da **API Secret** bude konfigurisano u podešavanjima plugina.

#### Jednostavni SSO

Podaci o korisniku (ime, email, avatar) se šalju sa klijentske strane bez kriptografskog potpisa. To je lakše za postavljanje pošto ne zahteva **API Secret**, ali je manje sigurno jer se podaci o korisniku ne verifikuju na serverskoj strani.

#### Nema

Nema SSO integracije. Korisnici komentarišu anonimno ili se moraju zasebno prijaviti na FastComments. Koristite ovo ako ne želite da Moodle korisnički nalozi budu povezani sa komentarima.