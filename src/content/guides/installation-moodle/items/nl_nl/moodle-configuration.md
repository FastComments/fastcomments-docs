De plugin-instellingenpagina bevindt zich op **Site Administration > Plugins > Local plugins > FastComments**. De beschikbare opties zijn:

#### Tenant ID

Je FastComments Tenant ID. Je vindt dit in het <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> onder je accountinstellingen.

#### API Secret

Je API Secret key, vereist voor Secure SSO modus. Je vindt dit op <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Kies hoe gebruikers worden geauthenticeerd. Zie de sectie [SSO Modes](#moodle-sso-modes) voor details over elke optie.

- **Secure** (aanbevolen) - server-side HMAC-SHA256 ondertekende authenticatie
- **Simple** - client-side gebruikersgegevens zonder handtekening
- **None** - anoniem reageren, geen Moodle-login-integratie

#### Page Contexts

Bepaalt waar reacties verschijnen:

- **Course pages** - reacties op de hoofdpagina's van cursussen
- **Module/activity pages** - reacties op individuele activiteiten en bronnen
- **Both** - reacties op alle paginatypen

#### Commenting Style

Kies de reactie-ervaring. Zie [Commenting Styles](#moodle-commenting-styles) voor screenshots van elke modus.

- **Comments** - standaard geneste reactiewidget onder de pagina-inhoud
- **Collab Chat** - inline discussies bij tekstselectie met aanwezigheidsindicatoren
- **Both** - reacties en collab chat tegelijk actief

#### CDN URL

De FastComments CDN URL. Standaard is dit `https://cdn.fastcomments.com`. Wijzig dit naar de EU CDN URL als je data in de EU-regio wordt gehost.