Stranica sa podešavanjima dodatka nalazi se na **Site Administration > Plugins > Local plugins > FastComments**. Dostupne opcije su:

#### Tenant ID

Vaš FastComments Tenant ID. Pronađite ga u <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments kontrolnom panelu</a> u podešavanjima vašeg naloga.

#### API Secret

Vaš API Secret ključ, potreban za Secure SSO režim. Pronađite ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Izaberite način na koji se korisnici autentifikuju. Pogledajte odeljak [SSO Modes](#moodle-sso-modes) za detalje o svakoj opciji.

- **Secure** (preporučeno) - autentifikacija potpisana HMAC-SHA256 na strani servera
- **Simple** - podaci o korisniku na strani klijenta bez potpisa
- **None** - anonimno komentarišanje, bez integracije sa Moodle prijavom

#### Page Contexts

Kontroliše gde se komentari pojavljuju:

- **Course pages** - komentari na glavnim stranicama kurseva
- **Module/activity pages** - komentari na pojedinačnim aktivnostima i resursima
- **Both** - komentari na svim tipovima stranica

#### Commenting Style

Izaberite iskustvo komentarišanja. Pogledajte [Commenting Styles](#moodle-commenting-styles) za snimke ekrana svakog režima.

- **Comments** - standardni vidžet za komentare u nizu ispod sadržaja stranice
- **Collab Chat** - diskusije o izabranom tekstu sa indikatorima prisustva
- **Both** - komentari i collab chat aktivni istovremeno

#### CDN URL

FastComments CDN URL. Podrazumevano je `https://cdn.fastcomments.com`. Promenite ovo na EU CDN URL ako su vaši podaci hostovani u EU regionu.