Stranica postavki dodatka nalazi se na **Administracija stranice > Dodaci > Lokalni dodaci > FastComments**. Dostupne opcije su:

#### ID zakupca

Vaš FastComments Tenant ID. Pronađite ovo na <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments nadzornoj ploči</a> u postavkama vašeg računa.

#### API tajna

Vaš API Secret ključ, potreban za Secure SSO način. Pronađite ovo na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moj račun > API tajna</a>.

#### SSO način

Odaberite kako se korisnici autentificiraju. Pogledajte odjeljak [SSO načini](#moodle-sso-modes) za detalje o svakoj opciji.

- **Sigurno** (preporučeno) - autentikacija potpisana HMAC-SHA256 na strani poslužitelja
- **Jednostavno** - korisnički podaci na strani klijenta bez potpisa
- **Nijedan** - anonimno komentiranje, bez integracije prijave u Moodle

#### Konteksti stranice

Kontrolira gdje se komentari pojavljuju:

- **Stranice kolegija** - komentari na glavnim stranicama kolegija
- **Stranice modula/aktivnosti** - komentari na pojedinačnim aktivnostima i resursima
- **Oba** - komentari na svim vrstama stranica

#### Stil komentiranja

Odaberite iskustvo komentiranja. Pogledajte [Stilovi komentiranja](#moodle-commenting-styles) za snimke zaslona svakog moda.

- **Komentari** - standardni ugniježđeni widget za komentare ispod sadržaja stranice
- **Collab Chat** - rasprave unutar odabranog teksta s indikatorima prisutnosti
- **Oba** - komentari i Collab Chat aktivni istovremeno

#### CDN URL

FastComments CDN URL. Zadano je `https://cdn.fastcomments.com`. Promijenite ovo na EU CDN URL ako su vaši podaci hostani u EU regiji.