Alle instellingen bevinden zich onder `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Vereist

- **Tenant ID** - Uw FastComments Tenant ID. Dit vindt u onder [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Vereist voor Secure SSO, webhookverificatie en pagina-synchronisatie. Te vinden onder [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Commentaarstijl

Kies de widget die past bij hoe u wilt dat mensen op uw site met elkaar praten.

- **Live Comments** - Realtime geneste reacties.
- **Streaming Chat** - Live chat-interface, geschikt voor evenementen en livestreams.
- **Collab Chat** - Tekstselectie-annotatie op het hoofdinhoudsgebied. Bezoekers markeren tekst en starten een discussie in de context.
- **Collab Chat + Comments** - Zowel collab chat als standaardreacties op dezelfde pagina.

## SSO-modus

- **None** - Geen SSO. Gebruikers reageren als gasten of maken een FastComments-account aan.
- **Simple** - Geeft Drupal-gebruikersgegevens (naam, e-mail, avatar) door aan FastComments zonder server-side verificatie.
- **Secure** - Gebruikt HMAC-SHA256 om Drupal-gebruikers te verifiëren met FastComments. Aanbevolen wanneer u een API Secret hebt geconfigureerd.

Zie de `Single Sign-On (SSO)` sectie voor details.

## Overige instellingen

- **CDN URL** - Standaard `https://cdn.fastcomments.com`.
- **Site URL** - Standaard `https://fastcomments.com`.
- **Email notifications** - Stuurt een e-mail naar de auteur van de inhoud wanneer er een nieuwe reactie op hun inhoud wordt geplaatst.

Voor EU-gegevensresidentie, zie de sectie `EU Data Residency`.