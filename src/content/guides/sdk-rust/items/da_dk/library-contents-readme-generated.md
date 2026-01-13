---
FastComments Rust SDK består af flere moduler:

- **Client Module** - Auto-generated API client for FastComments REST APIs
  - Komplette typedefinitioner for alle API-modeller
  - Både autentificerede (`DefaultApi`) og offentlige (`PublicApi`) endpoints
  - Fuld async/await-support med tokio
  - Se [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) for detaljeret API-dokumentation

- **SSO Module** - Server-side Single Sign-On utilities
  - Sikker token-generering til brugerautentificering
  - Understøttelse af både simple og sikre SSO-tilstande
  - HMAC-SHA256 baseret token-signering

- **Core Types** - Shared type definitions and utilities
  - Kommentarmodeller og metadata-strukturer
  - Bruger- og tenant-konfigurationer
  - Hjælpefunktioner til almindelige operationer
---