The plugin settings page is at **Site Administration > Plugins > Local plugins > FastComments**. Dostupne opcije su:

#### Tenant ID

Vaš FastComments Tenant ID. Pronađite ovo u <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> u podešavanjima naloga.

#### API Secret

Vaš API Secret ključ, potreban za Secure SSO režim. Pronađite ovo na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Izaberite kako se korisnici autentifikuju. Pogledajte odeljak [SSO Modes](#moodle-sso-modes) za detalje o svakoj opciji.

- **Secure** (recommended) - autentifikacija potpisana na serverskoj strani pomoću HMAC-SHA256
- **Simple** - podaci o korisniku na klijentskoj strani bez potpisa
- **None** - anonimno komentarisanje, bez integracije Moodle prijave

#### Page Contexts

Kontroliše gde se komentari pojavljuju:

- **Course pages** - komentari na glavnim stranicama kurseva
- **Module/activity pages** - komentari na pojedinačnim aktivnostima i resursima
- **Both** - komentari na svim tipovima stranica

#### Commenting Style

Izaberite iskustvo komentarisanja. Pogledajte [Commenting Styles](#moodle-commenting-styles) za snimke ekrana svakog režima.

- **Comments** - standardni ugnježdeni widget za komentare ispod sadržaja stranice
- **Collab Chat** - inline diskusije selekcijom teksta sa indikatorima prisutnosti
- **Both** - komentari i collab chat aktivni zajedno

#### CDN URL

URL FastComments CDN-a. Podrazumevano je `https://cdn.fastcomments.com`. Promenite ovo na EU CDN URL ako su vaši podaci hostovani u EU regionu.