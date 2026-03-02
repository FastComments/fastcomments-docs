The plugin settings page is at **Administracja witryną > Wtyczki > Wtyczki lokalne > FastComments**. The available options are:

#### Tenant ID

Your FastComments Tenant ID. Find this in the <a href="https://fastcomments.com/auth/my-account" target="_blank">panel FastComments</a> under your account settings.

#### API Secret

Your API Secret key, required for Secure SSO mode. Find this at <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Moje konto > API Secret</a>.

#### SSO Mode

Choose how users are authenticated. See the [SSO Modes](#moodle-sso-modes) section for details on each option.

- **Secure** (recommended) - uwierzytelnianie podpisane po stronie serwera przy użyciu HMAC-SHA256
- **Simple** - dane użytkownika po stronie klienta bez podpisu
- **None** - anonimowe komentowanie, brak integracji z logowaniem Moodle

#### Page Contexts

Controls where comments appear:

- **Course pages** - komentarze na stronach głównych kursów
- **Module/activity pages** - komentarze na stronach pojedynczych aktywności i zasobów
- **Both** - komentarze na wszystkich typach stron

#### Commenting Style

Choose the commenting experience. See [Commenting Styles](#moodle-commenting-styles) for screenshots of each mode.

- **Comments** - standardowy wielowątkowy widżet komentarzy poniżej treści strony
- **Collab Chat** - inline text selection discussions with presence indicators
- **Both** - komentarze i Collab Chat aktywne jednocześnie

#### CDN URL

The FastComments CDN URL. Defaults to `https://cdn.fastcomments.com`. Change this to the EU CDN URL if your data is hosted in the EU region.