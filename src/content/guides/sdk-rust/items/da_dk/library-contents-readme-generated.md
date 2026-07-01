The FastComments Rust SDK består af flere moduler:

- **Client Module** - API-klient for FastComments REST API'er
  - Komplet type definitioner for alle API-modeller
  - Tre API-klienter der dækker alle FastComments‑metoder:
    - `default_api` (**DefaultApi**) - API‑nøgleautentificerede metoder til server‑side brug
    - `public_api` (**PublicApi**) - offentlige metoder uden API‑nøgle, som er sikre at kalde fra browsere og mobilapps
    - `moderation_api` (**ModerationApi**) - en omfattende suite af live og hurtige moderations‑API'er. Hver Moderationsmetode accepterer en `sso`‑parameter og kan autentificere via SSO eller en FastComments.com sessionscookie.
  - Fuld async/await support med tokio
  - Se [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) for detaljeret API‑dokumentation

- **SSO Module** - Server‑side Single Sign‑On værktøjer
  - Sikker token‑generering til brugerautentificering
  - Understøttelse af både simple og sikre SSO‑tilstande
  - HMAC‑SHA256‑baseret token‑signering

- **Core Types** - Delte type definitioner og værktøjer
  - Kommentar‑modeller og metadata‑strukturer
  - Bruger‑ og lejer‑konfigurationer
  - Hjælpefunktioner til almindelige operationer