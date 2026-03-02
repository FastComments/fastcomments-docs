Stranica za podešavanje plugina nalazi se na **Administracija sajta > Dodaci > Lokalni dodaci > FastComments**. Dostupne opcije su:

#### ID zakupca

Vaš FastComments Tenant ID. Pronađite ovo u <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments kontrolnoj tabli</a> u podešavanjima naloga.

#### API tajni ključ

Vaš API Secret ključ, potreban za Secure SSO režim. Pronađite ovo na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moj nalog > API tajni ključ</a>.

#### SSO režim

Izaberite kako se korisnici autentifikuju. Pogledajte odeljak [SSO Modes](#items-moodle-sso-modes) za detalje o svakoj opciji.

- **Secure** (preporučeno) - serverska HMAC-SHA256 potpisana autentifikacija
- **Simple** - podaci o korisniku na strani klijenta bez potpisa
- **None** - anonimno komentarisanje, bez integracije Moodle prijave

#### Konteksti stranica

Kontroliše gde se prikazuju komentari:

- **Stranice kursa** - komentari na glavnim stranicama kursa
- **Stranice modula/aktivnosti** - komentari na pojedinačnim aktivnostima i resursima
- **Oba** - komentari na svim tipovima stranica

#### Stil komentarisanja

Izaberite iskustvo komentarisanja. Pogledajte [Commenting Styles](#items-moodle-commenting-styles) za snimke ekrana svakog režima.

- **Komentari** - standardni ugnježdeni widget za komentare ispod sadržaja stranice
- **Collab Chat** - diskusije direktno u tekstu pomoću selekcije teksta sa indikatorima prisutnosti
- **Oba** - komentari i Collab Chat aktivni istovremeno

#### CDN URL

URL FastComments CDN-a. Podrazumevano je `https://cdn.fastcomments.com`. Promenite ovo na EU CDN URL ako su vaši podaci hostovani u EU regionu.