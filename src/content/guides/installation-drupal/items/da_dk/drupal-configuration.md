Alle indstillinger findes under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Påkrævet

- **Tenant ID** - Din FastComments Tenant ID. Find den under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Påkrævet for Secure SSO, webhook-verificering og side-synkronisering. Findes under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Kommentarstil

Vælg den widget, der matcher, hvordan du ønsker, at folk skal kommunikere på dit site.

- **Live Comments** - Trådede kommentarer i realtid.
- **Streaming Chat** - Live chat-interface, godt til events og livestreams.
- **Collab Chat** - Annotation ved tekstmarkering i hovedindholdsområdet. Besøgende fremhæver tekst og starter en diskussion i kontekst.
- **Collab Chat + Comments** - Både collab chat og standardkommentarer på samme side.

## SSO-tilstand

- **None** - Ingen SSO. Brugere kommenterer som gæster eller opretter en FastComments-konto.
- **Simple** - Sender Drupal-brugeroplysninger (navn, e-mail, avatar) til FastComments uden server-side verifikation.
- **Secure** - Bruger HMAC-SHA256 til at verificere Drupal-brugere med FastComments. Anbefales, når du har konfigureret en API Secret.

Se afsnittet `Single Sign-On (SSO)` for detaljer.

## Andre indstillinger

- **CDN URL** - Som standard er `https://cdn.fastcomments.com`.
- **Site URL** - Som standard er `https://fastcomments.com`.
- **Email notifications** - Send en e-mail til indholdsforfatteren, når der postes en ny kommentar på deres indhold.

For EU-dataresidens, se afsnittet `EU Data Residency`.

---