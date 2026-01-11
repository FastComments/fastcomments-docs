---
FastComments Rust SDK składa się z kilku modułów:

- **Moduł klienta** - Automatycznie wygenerowany klient API dla interfejsów REST FastComments
  - Pełne definicje typów dla wszystkich modeli API
  - Zarówno uwierzytelnione (`DefaultApi`) i publiczne (`PublicApi`) punkty końcowe
  - Pełne wsparcie dla async/await z tokio
  - Zobacz [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) dla szczegółowej dokumentacji API

- **Moduł SSO** - Narzędzia Single Sign-On po stronie serwera
  - Bezpieczne generowanie tokenów do uwierzytelniania użytkowników
  - Wsparcie zarówno dla prostych, jak i zabezpieczonych trybów SSO
  - Podpisywanie tokenów oparte na HMAC-SHA256

- **Typy podstawowe** - Wspólne definicje typów i narzędzia
  - Modele komentarzy i struktury metadanych
  - Konfiguracje użytkowników i tenantów
  - Funkcje pomocnicze dla typowych operacji
---