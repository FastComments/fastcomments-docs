Die Einstellungsseite des Plugins befindet sich unter **Site Administration > Plugins > Local plugins > FastComments**. Die verfügbaren Optionen sind:

#### Tenant ID

Your FastComments Tenant ID. Find this in the <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments-Dashboard</a> under your account settings.

#### API Secret

Your API Secret key, required for Secure SSO mode. Find this at <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mein Konto > API Secret</a>.

#### SSO Mode

Wählen Sie, wie Benutzer authentifiziert werden. Siehe den [SSO Modes](#items-moodle-sso-modes) Abschnitt für Details zu jeder Option.

- **Secure** (recommended) - server-side HMAC-SHA256 signed authentication
- **Simple** - client-side user data without signature
- **None** - anonymous commenting, no Moodle login integration

#### Page Contexts

Steuert, wo Kommentare erscheinen:

- **Course pages** - comments on course main pages
- **Module/activity pages** - comments on individual activities and resources
- **Both** - comments on all page types

#### Commenting Style

Wählen Sie das Kommentiererlebnis. Siehe [Commenting Styles](#items-moodle-commenting-styles) für Screenshots der einzelnen Modi.

- **Comments** - standard threaded comment widget below page content
- **Collab Chat** - inline text selection discussions with presence indicators
- **Both** - comments and collab chat active together

#### CDN URL

The FastComments CDN URL. Defaults to `https://cdn.fastcomments.com`. Change this to the EU CDN URL if your data is hosted in the EU region.