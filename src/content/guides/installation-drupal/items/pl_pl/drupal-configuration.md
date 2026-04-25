Wszystkie ustawienia znajdują się w `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Wymagane

- **Tenant ID** - Twój FastComments Tenant ID. Znajdź to w [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Wymagane dla Secure SSO, weryfikacji webhooków oraz synchronizacji stron. Znajduje się w [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Styl komentowania

Wybierz widget, który odpowiada temu, jak chcesz, aby ludzie rozmawiali na Twojej stronie.

- **Live Comments** - Komentarze wątkowe w czasie rzeczywistym.
- **Streaming Chat** - Interfejs czatu na żywo, dobry na wydarzenia i transmisje na żywo.
- **Collab Chat** - Adnotacje poprzez zaznaczanie tekstu na głównym obszarze treści. Odwiedzający zaznaczają tekst i zaczynają dyskusję w kontekście.
- **Collab Chat + Comments** - Zarówno collab chat, jak i standardowe komentarze na tej samej stronie.

## Tryb SSO

- **None** - Brak SSO. Użytkownicy komentują jako goście lub tworzą konto FastComments.
- **Simple** - Przesyła informacje użytkownika Drupala (imię, email, avatar) do FastComments bez weryfikacji po stronie serwera.
- **Secure** - Używa HMAC-SHA256 do weryfikacji użytkowników Drupala z FastComments. Zalecane, gdy masz skonfigurowane API Secret.

Zobacz sekcję `Single Sign-On (SSO)` po szczegóły.

## Inne ustawienia

- **CDN URL** - Domyślnie `https://cdn.fastcomments.com`.
- **Site URL** - Domyślnie `https://fastcomments.com`.
- **Email notifications** - Wyślij e-mail do autora treści, gdy na jego treści pojawi się nowy komentarz.

W sprawie przechowywania danych w UE zobacz sekcję `EU Data Residency`.