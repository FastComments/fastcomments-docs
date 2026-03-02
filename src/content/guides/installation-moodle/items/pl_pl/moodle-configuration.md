Strona ustawień wtyczki znajduje się w **Site Administration > Plugins > Local plugins > FastComments**. Dostępne opcje to:

#### Tenant ID

Twój FastComments Tenant ID. Znajdź go w <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> w ustawieniach konta.

#### API Secret

Twój klucz API Secret, wymagany dla trybu Secure SSO. Znajdź go w <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Wybierz sposób uwierzytelniania użytkowników. Zobacz sekcję [SSO Modes](#items-moodle-sso-modes) po szczegóły każdego z wariantów.

- **Secure** (zalecane) - uwierzytelnianie po stronie serwera z podpisem HMAC-SHA256
- **Simple** - dane użytkownika po stronie klienta bez podpisu
- **None** - anonimowe komentowanie, brak integracji z logowaniem Moodle

#### Page Contexts

Kontroluje, gdzie pojawiają się komentarze:

- **Course pages** - komentarze na stronach głównych kursów
- **Module/activity pages** - komentarze na stronach poszczególnych aktywności i zasobów
- **Both** - komentarze na wszystkich typach stron

#### Commenting Style

Wybierz sposób komentowania. Zobacz [Commenting Styles](#items-moodle-commenting-styles) po zrzuty ekranu każdego trybu.

- **Comments** - standardowy wątkowany widżet komentarzy poniżej treści strony
- **Collab Chat** - dyskusje inline po zaznaczeniu tekstu z wskaźnikami obecności
- **Both** - jednoczesne aktywowanie komentarzy i collab chatu

#### CDN URL

Adres CDN FastComments. Domyślnie `https://cdn.fastcomments.com`. Zmień to na adres CDN dla UE, jeśli Twoje dane są hostowane w regionie UE.