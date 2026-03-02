Stranica postavki dodatka nalazi se na **Site Administration > Plugins > Local plugins > FastComments**. Dostupne opcije su:

#### Tenant ID

Your FastComments Tenant ID. Pronađite ga u <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> u postavkama vašeg računa.

#### API Secret

Your API Secret key, required for Secure SSO mode. Pronađite ga na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Choose how users are authenticated. See the [SSO Modes](#moodle-sso-modes) section for details on each option.

- **Secure** (recommended) - serverska autentikacija potpisana HMAC-SHA256
- **Simple** - klijentski podaci korisnika bez potpisa
- **None** - anonimno komentiranje, bez integracije Moodle prijave

#### Page Contexts

Controls where comments appear:

- **Course pages** - komentari na glavnim stranicama tečajeva
- **Module/activity pages** - komentari na pojedinačnim aktivnostima i resursima
- **Both** - komentari na svim vrstama stranica

#### Commenting Style

Choose the commenting experience. See [Commenting Styles](#moodle-commenting-styles) for screenshots of each mode.

- **Comments** - standardni widget komentara u nitima ispod sadržaja stranice
- **Collab Chat** - rasprave unutar teksta putem označavanja s indikatorima prisutnosti
- **Both** - comments and collab chat active together

#### CDN URL

The FastComments CDN URL. Defaults to `https://cdn.fastcomments.com`. Change this to the EU CDN URL if your data is hosted in the EU region.