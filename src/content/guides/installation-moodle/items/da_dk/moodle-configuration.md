Siden for pluginindstillinger findes under **Site Administration > Plugins > Local plugins > FastComments**. Følgende indstillinger er tilgængelige:

#### Tenant ID

Dit FastComments Tenant ID. Find dette i <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> under dine kontoindstillinger.

#### API Secret

Din API Secret-nøgle, påkrævet for Secure SSO-tilstand. Find dette på <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Min konto > API Secret</a>.

#### SSO Mode

Vælg, hvordan brugere godkendes. Se afsnittet [SSO Modes](#moodle-sso-modes) for detaljer om hver mulighed.

- **Secure** (recommended) - server-side HMAC-SHA256 signed authentication
- **Simple** - client-side user data without signature
- **None** - anonymous commenting, no Moodle login integration

#### Page Contexts

Styrer, hvor kommentarer vises:

- **Course pages** - comments on course main pages
- **Module/activity pages** - comments on individual activities and resources
- **Both** - comments on all page types

#### Commenting Style

Vælg kommenteringsoplevelsen. Se [Commenting Styles](#moodle-commenting-styles) for skærmbilleder af hver tilstand.

- **Comments** - standard threaded comment widget below page content
- **Collab Chat** - inline text selection discussions with presence indicators
- **Both** - comments and collab chat active together

#### CDN URL

FastComments CDN URL. Standard er `https://cdn.fastcomments.com`. Skift dette til EU-CDN-URL'en, hvis dine data hostes i EU-regionen.