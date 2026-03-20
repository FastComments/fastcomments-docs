Gå til **Administration > Konfiguration > Indhold > FastComments** (`/admin/config/content/fastcomments`).

### Indstillinger

- **Tenant ID** (påkrævet) - Dit FastComments Tenant ID. Find dette under [Indstillinger > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Påkrævet til Secure SSO, webhook-verifikation og side-synkronisering. Findes under [Indstillinger > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO-mode** - Single Sign-On-integration:
  - **Ingen** - Ingen SSO; brugere kommenterer som gæster eller opretter FastComments-konti.
  - **Simple** - Videregiver Drupal-brugerinfo (navn, e-mail, avatar) til FastComments uden serverside-verifikation.
  - **Secure** - Bruger HMAC-SHA256-verifikation til sikkert at autentificere Drupal-brugere med FastComments (anbefales).
- **Kommenteringsstil** - Typen af widget der vises:
  - **Live Comments** - Realtids, trådede kommentarer.
  - **Streaming Chat** - Live chat-grænseflade.
  - **Collab Chat** - Samarbejdende tekstudvælgelses-annotering på hovedindholdsområdet.
  - **Collab Chat + Comments** - Både collab chat og standardkommentarer.
- **CDN URL** - FastComments CDN URL (standard: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (standard: `https://fastcomments.com`).
- **E-mail-notifikationer** - Send en e-mail til indholdets forfattere, når en ny kommentar er postet på deres indhold.

### Tilføj kommentarer til indholdstyper

Tilføj feltet **FastComments** til dine indholdstyper via **Struktur > Indholdstyper > [type] > Administrer felter**. Feltet har en status-til-/fra-knap og en valgfri brugerdefineret identifikator per enhed.

### EU-dataopbevaring

For EU-dataopbevaring, opdater:
- **CDN URL** til `https://cdn-eu.fastcomments.com`
- **Site URL** til `https://eu.fastcomments.com`