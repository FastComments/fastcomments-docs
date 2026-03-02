De plugin-instellingspagina bevindt zich op **Site-administratie > Plugins > Lokale plugins > FastComments**. De beschikbare opties zijn:

#### Tenant ID

Uw FastComments Tenant ID. U vindt dit op het <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments-dashboard</a> onder uw accountinstellingen.

#### API Secret

Uw API Secret-sleutel, vereist voor Secure SSO-modus. U vindt deze op <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mijn account &gt; API Secret</a>.

#### SSO Mode

Kies hoe gebruikers worden geverifieerd. Zie de sectie [SSO-modi](#moodle-sso-modes) voor details over elke optie.

- **Secure** (recommended) - server-side HMAC-SHA256 ondertekende authenticatie
- **Simple** - client-side gebruikersgegevens zonder handtekening
- **None** - anoniem reageren, geen Moodle-loginintegratie

#### Page Contexts

Bepaal waar reacties verschijnen:

- **Course pages** - reacties op hoofdpagina's van cursussen
- **Module/activity pages** - reacties op individuele activiteiten en bronnen
- **Both** - reacties op alle paginatypen

#### Commenting Style

Kies de commentaarervaring. Zie [Commentaarstijlen](#moodle-commenting-styles) voor schermafbeeldingen van elke modus.

- **Comments** - standaard draadgestructureerde commentaar-widget onder de paginainhoud
- **Collab Chat** - inline discussies op geselecteerde tekst met aanwezigheidsindicatoren
- **Both** - comments en collab chat tegelijk actief

#### CDN URL

De FastComments CDN-URL. Standaard `https://cdn.fastcomments.com`. Wijzig dit naar de EU-CDN-URL als uw gegevens in de EU-regio worden gehost.