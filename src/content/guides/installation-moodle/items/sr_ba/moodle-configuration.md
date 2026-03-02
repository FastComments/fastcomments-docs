Stranica sa postavkama dodatka se nalazi na **Site Administration > Plugins > Local plugins > FastComments**. Dostupne opcije su:

#### Tenant ID

Vaš FastComments Tenant ID. Pronađite ga u <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> pod postavkama naloga.

#### API Secret

Vaš API Secret ključ, potreban za Secure SSO mode. Pronađite ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Izaberite kako su korisnici autentifikovani. Pogledajte sekciju [SSO Modes](#items-moodle-sso-modes) za detalje o svakoj opciji.

- **Secure** (preporučeno) - serverski HMAC-SHA256 potpis za autentifikaciju
- **Simple** - klijentski podaci o korisniku bez potpisa
- **None** - anonimno komentarisanje, bez integracije prijave na Moodle

#### Page Contexts

Kontroliše gdje se komentari prikazuju:

- **Course pages** - komentari na glavnim stranicama kursa
- **Module/activity pages** - komentari na pojedinačnim aktivnostima i resursima
- **Both** - komentari na svim tipovima stranica

#### Commenting Style

Izaberite iskustvo komentarisanja. Pogledajte [Commenting Styles](#items-moodle-commenting-styles) za snimke ekrana svakog režima.

- **Comments** - standardni ugnježdeni widget za komentare ispod sadržaja stranice
- **Collab Chat** - diskusije direktno u tekstu putem selekcije, sa indikatorima prisustva
- **Both** - Comments i Collab Chat aktivni zajedno

#### CDN URL

The FastComments CDN URL. Zadano je `https://cdn.fastcomments.com`. Promijenite ovo na EU CDN URL ako su vaši podaci hostovani u EU regiji.