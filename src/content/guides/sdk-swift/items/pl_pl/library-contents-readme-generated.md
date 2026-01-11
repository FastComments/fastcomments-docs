---
FastComments Swift SDK składa się z kilku modułów:

- **Moduł klienta** - Automatycznie wygenerowany klient API dla FastComments REST APIs
  - Kompletne definicje typów dla wszystkich modeli API
  - Zarówno uwierzytelnione (`DefaultAPI`), jak i publiczne (`PublicAPI`) punkty końcowe
  - Pełne wsparcie dla async/await
  - Zobacz [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) w celu uzyskania szczegółowej dokumentacji API

- **Moduł SSO** - Narzędzia Single Sign-On po stronie serwera
  - Bezpieczne generowanie tokenów do uwierzytelniania użytkowników
  - Wsparcie zarówno dla prostego, jak i bezpiecznego trybu SSO
  - Podpisywanie tokenów oparte na HMAC-SHA256 przy użyciu CryptoKit
---