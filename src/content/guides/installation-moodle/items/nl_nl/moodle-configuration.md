De plugin-instellingenpagina bevindt zich op **Site Administration > Plugins > Local plugins > FastComments**. De beschikbare opties zijn:

#### Tenant ID

Uw FastComments Tenant ID. Vind dit in het <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> onder uw accountinstellingen.

#### API Secret

Uw API Secret-sleutel, vereist voor Secure SSO-modus. Vind deze op <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mijn account &gt; API Secret</a>.

#### SSO Mode

Kies hoe gebruikers worden geverifieerd. Zie de [SSO-modi](#moodle-sso-modes) sectie voor details over elke optie.

- **Beveiligd** (aanbevolen) - server-side HMAC-SHA256 ondertekende authenticatie
- **Eenvoudig** - client-side gebruikersgegevens zonder handtekening
- **Geen** - anoniem reageren, geen Moodle-loginintegratie

#### Page Contexts

Bepaalt waar reacties verschijnen:

- **Cursuspagina's** - reacties op de hoofdpagina's van de cursus
- **Module/activiteitspagina's** - reacties op individuele activiteiten en bronnen
- **Beide** - reacties op alle pagina-typen

#### Commenting Style

Kies de commentaarervaring. Zie [Commentaarstijlen](#moodle-commenting-styles) voor screenshots van elke modus.

- **Reacties** - standaard gelaagde reactiewidget onder de paginainhoud
- **Collab Chat** - inline discussies bij tekstselectie met aanwezigheidindicatoren
- **Beide** - reacties en Collab Chat tegelijk actief

#### CDN URL

De FastComments CDN URL. Standaard is `https://cdn.fastcomments.com`. Wijzig dit naar de EU CDN-URL als uw gegevens in de EU-regio worden gehost.