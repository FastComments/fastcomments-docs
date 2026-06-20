The FastComments Swift SDK składa się z kilku modułów:

- **Client Module** - Klient API dla FastComments REST API
  - Pełne definicje typów dla wszystkich modeli API
  - Metody uwierzytelnione (`DefaultAPI`), publiczne (`PublicAPI`) i moderacyjne (`ModerationAPI`)
  - Pełne wsparcie dla async/await
  - Zobacz [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) dla szczegółowej dokumentacji API

- **SSO Module** - Narzędzia Single Sign-On po stronie serwera
  - Bezpieczne generowanie tokenów do uwierzytelniania użytkowników
  - Obsługa zarówno prostego, jak i bezpiecznego trybu SSO
  - Podpisywanie tokenów oparte na HMAC-SHA256 przy użyciu CryptoKit