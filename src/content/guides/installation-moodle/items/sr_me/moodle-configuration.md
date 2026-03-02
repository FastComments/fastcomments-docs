Stranica sa podešavanjima dodatka se nalazi u **Administracija sajta > Dodaci > Lokalni dodaci > FastComments**. Dostupne opcije su:

#### Tenant ID

Vaš FastComments Tenant ID. Pronađite ga na <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments nadzornoj ploči</a> pod podešavanjima naloga.

#### API Secret

Vaš API Secret ključ, potreban za Secure SSO režim. Pronađite ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moj nalog > API Secret</a>.

#### SSO Mode

Izaberite kako se korisnici autentifikuju. Pogledajte odeljak [SSO Modes](#items-moodle-sso-modes) za detalje o svakoj opciji.

- **Secure** (preporučeno) - autentifikacija potpisana HMAC-SHA256 na serverskoj strani
- **Simple** - korisnički podaci na klijentskoj strani bez potpisa
- **None** - anonimno komentarisanje, bez integracije sa Moodle prijavom

#### Page Contexts

Kontroliše gde se komentari prikazuju:

- **Course pages** - komentari na glavnim stranicama kursa
- **Module/activity pages** - komentari na pojedinačnim aktivnostima i resursima
- **Both** - komentari na svim tipovima stranica

#### Commenting Style

Izaberite način komentarisanja. Pogledajte [Commenting Styles](#items-moodle-commenting-styles) za snimke ekrana svakog režima.

- **Comments** - standardni ugnježdeni vidžet komentara ispod sadržaja stranice
- **Collab Chat** - diskusije preko odabira teksta u liniji sa indikatorima prisutnosti
- **Both** - komentari i collab chat aktivni istovremeno

#### CDN URL

FastComments CDN URL. Podrazumevano je `https://cdn.fastcomments.com`. Promijenite ovo na EU CDN URL ako se vaši podaci nalaze u EU regionu.