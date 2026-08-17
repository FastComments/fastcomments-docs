---
The FastComments Rust SDK consists of several modules:

- **Client Module** - Klient API dla FastComments REST APIs
  - Pełne definicje typów dla wszystkich modeli API
  - Trzy klientów API obejmujących wszystkie metody FastComments:
    - `default_api` (**DefaultApi**) - Metody uwierzytelniane kluczem API do użycia po stronie serwera
    - `public_api` (**PublicApi**) - publiczne, nie wymagające klucza API metody, które są bezpieczne do wywoływania z przeglądarek i aplikacji mobilnych
    - `moderation_api` (**ModerationApi**) - rozbudowany zestaw szybkich i bieżących API moderacji. Każda metoda Moderacji przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesji FastComments.com.
  - Pełne wsparcie async/await z tokio
  - Zobacz [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) po szczegółową dokumentację API

- **SSO Module** - Narzędzia Single Sign-On po stronie serwera
  - Bezpieczne generowanie tokenów do uwierzytelniania użytkowników
  - Obsługa zarówno prostych, jak i bezpiecznych trybów SSO
  - Podpisywanie tokenów oparte na HMAC-SHA256

- **Core Types** - Wspólne definicje typów i narzędzia
  - Modele komentarzy i struktury metadanych
  - Konfiguracje użytkowników i najemców
  - Funkcje pomocnicze dla typowych operacji
---