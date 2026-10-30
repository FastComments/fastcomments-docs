All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Wymagane

- **Tenant ID** - Twój FastComments Tenant ID. Znajdź go pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Wymagany dla Secure SSO, weryfikacji webhooków i synchronizacji stron. Znajduje się pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Przechowywanie sekretu API

Moduł odczytuje sekret API z pierwszego z tych źródeł, które ma wartość:

1. **Key module** - Zainstaluj [Key](https://www.drupal.org/project/key), utwórz klucz *Authentication* i wybierz go w **API Secret key** w formularzu ustawień. Klucz może być oparty na zmiennej środowiskowej lub pliku poza katalogiem głównym witryny.
2. **settings.php** - Dodaj `$config['fastcomments.settings']['api_secret'] = 'your-secret';` do `settings.php` lub `settings.local.php`. Sekret nigdy nie trafia do eksportowanej konfiguracji.
3. **The settings form** - Wprowadź go w polu **API Secret**. To przechowuje wartość w `fastcomments.settings`, więc trafia do eksportów konfiguracji. Raport statusu Drupala ostrzega, gdy jest to aktywne źródło.

Formularz pokazuje, które źródło jest aktywne. Pozostawienie pola pustego zachowuje przechowywany sekret. Zaznacz **Remove the API Secret stored in configuration**, aby usunąć go po przeniesieniu sekretu do modułu Key lub `settings.php`.

## Styl komentowania

Wybierz widżet, który odpowiada temu, jak chcesz, aby ludzie rozmawiali na Twojej stronie.

- **Live Comments** - Komentarze w czasie rzeczywistym w formie wątków.
- **Streaming Chat** - Interfejs czatu na żywo, dobry na wydarzenia i transmisje na żywo.
- **Collab Chat** - Anotacja zaznaczonego tekstu w głównym obszarze treści. Odwiedzający podświetlają tekst i rozpoczynają dyskusję w kontekście.
- **Collab Chat + Comments** - Zarówno czat współpracy, jak i standardowe komentarze na tej samej stronie.

## Tryb SSO

- **None** - Brak SSO. Użytkownicy komentują jako goście lub tworzą konto FastComments.
- **Simple** - Przekazuje informacje o użytkowniku Drupal (imię, e-mail, awatar) do FastComments bez weryfikacji po stronie serwera.
- **Secure** - Używa HMAC‑SHA256 do weryfikacji użytkowników Drupal w FastComments. Zalecane, gdy skonfigurowany jest sekret API.

Zobacz sekcję `Single Sign-On (SSO)` po szczegóły.

## Inne ustawienia

- **CDN URL** - Domyślnie `https://cdn.fastcomments.com`.
- **Site URL** - Domyślnie `https://fastcomments.com`.
- **Email notifications** - Wyślij e‑mail do autora treści, gdy nowy komentarz zostanie opublikowany pod jego treścią.

W przypadku rezydencji danych w UE, zobacz sekcję `EU Data Residency`.

---