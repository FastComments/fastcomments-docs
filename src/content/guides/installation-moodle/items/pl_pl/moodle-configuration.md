Strona ustawień wtyczki znajduje się pod adresem **Site Administration > Plugins > Local plugins > FastComments**. Dostępne opcje to:

#### Tenant ID

Twój FastComments Tenant ID. Znajdziesz go w <a href="https://fastcomments.com/auth/my-account" target="_blank">panelu FastComments</a> w ustawieniach konta.

#### API Secret

Twój klucz API Secret, wymagany w trybie Secure SSO. Znajdziesz go pod adresem <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Wybierz sposób uwierzytelniania użytkowników. Szczegóły każdej opcji znajdziesz w sekcji [SSO Modes](#moodle-sso-modes).

- **Secure** (zalecane) - uwierzytelnianie podpisywane po stronie serwera za pomocą HMAC-SHA256
- **Simple** - dane użytkownika po stronie klienta bez podpisu
- **None** - komentowanie anonimowe, bez integracji z logowaniem Moodle

#### Page Contexts

Kontroluje, gdzie pojawiają się komentarze:

- **Course pages** - komentarze na głównych stronach kursów
- **Module/activity pages** - komentarze na stronach poszczególnych aktywności i zasobów
- **Both** - komentarze na wszystkich typach stron

#### Commenting Style

Wybierz sposób komentowania. Zrzuty ekranu każdego trybu znajdziesz w sekcji [Commenting Styles](#moodle-commenting-styles).

- **Comments** - standardowy wątkowany widżet komentarzy pod treścią strony
- **Collab Chat** - dyskusje w tekście z zaznaczania fragmentów oraz wskaźnikami obecności
- **Both** - komentarze i collab chat aktywne jednocześnie

#### CDN URL

Adres URL CDN FastComments. Domyślnie `https://cdn.fastcomments.com`. Zmień go na adres URL CDN dla UE, jeśli Twoje dane są hostowane w regionie UE.