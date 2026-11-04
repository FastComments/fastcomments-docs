Pluginsindstillingssiden findes under **Site Administration > Plugins > Local plugins > FastComments**. De tilgængelige indstillinger er:

#### Tenant ID

Dit FastComments Tenant ID. Du finder det i <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments-dashboardet</a> under dine kontoindstillinger.

#### API Secret

Din API Secret-nøgle, som kræves til Secure SSO-tilstand. Du finder den under <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Vælg, hvordan brugere autentificeres. Se afsnittet [SSO Modes](#moodle-sso-modes) for detaljer om hver mulighed.

- **Secure** (anbefalet) - serverbaseret autentificering signeret med HMAC-SHA256
- **Simple** - brugerdata på klientsiden uden signatur
- **None** - anonym kommentering, ingen integration med Moodle-login

#### Page Contexts

Styrer, hvor kommentarer vises:

- **Course pages** - kommentarer på kursushovedsider
- **Module/activity pages** - kommentarer på individuelle aktiviteter og ressourcer
- **Both** - kommentarer på alle sidetyper

#### Commenting Style

Vælg kommentaroplevelsen. Se [Commenting Styles](#moodle-commenting-styles) for skærmbilleder af hver tilstand.

- **Comments** - standard trådet kommentar-widget under sideindholdet
- **Collab Chat** - inline diskussioner af tekstmarkeringer med tilstedeværelsesindikatorer
- **Both** - kommentarer og collab chat aktive samtidig

#### CDN URL

FastComments CDN-URL. Standardværdien er `https://cdn.fastcomments.com`. Ændr denne til EU CDN-URL, hvis dine data hostes i EU-regionen.