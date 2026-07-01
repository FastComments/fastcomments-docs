FastComments SDK udostępnia trzy klienty API:

### PublicAPI - Client-Safe Methods

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Nie wymagają klucza API
- Mogą używać tokenów SSO do uwierzytelniania
- Są ograniczane (rate‑limited) na użytkownika/urządzenie
- Są odpowiednie dla aplikacji skierowanych do końcowych użytkowników

**Example use case**: Przykładowy przypadek użycia: Pobieranie i tworzenie komentarzy w Twojej aplikacji iOS

### DefaultAPI - Server-Side Methods

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Wymagają Twojego klucza API FastComments
- Powinny być wywoływane TYLKO z kodu po stronie serwera
- Zapewniają pełny dostęp do danych FastComments
- Są ograniczane (rate‑limited) na najemcę

**Example use case**: Przykładowy przypadek użycia: Operacje administracyjne, masowy eksport danych, zarządzanie użytkownikami

### ModerationAPI - Moderator Dashboard Methods

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.
- Udostępnia rozbudowany zestaw szybkich i bieżących API moderacji
- Każda metoda `ModerationAPI` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesji FastComments.com

**Example use case**: Przykładowy przypadek użycia: Tworzenie doświadczenia moderacji dla moderatorów Twojej społeczności

**IMPORTANT**: WAŻNE: Nigdy nie udostępniaj swojego klucza API w kodzie po stronie klienta. Klucze API powinny być używane wyłącznie po stronie serwera.