Ga naar **Administratie > Configuratie > Inhoud > FastComments** (`/admin/config/content/fastcomments`).

### Instellingen

- **Tenant ID** (required) - Uw FastComments Tenant ID. Vind dit onder [Instellingen > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Vereist voor Secure SSO, webhook-verificatie en page sync. Te vinden onder [Instellingen > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Single Sign-On-integratie:
  - **None** - Geen SSO, gebruikers reageren als gasten of maken FastComments-accounts.
  - **Simple** - Stuurt Drupal-gebruikersinformatie (naam, e-mail, avatar) naar FastComments zonder server-side verificatie.
  - **Secure** - Gebruikt HMAC-SHA256-verificatie om Drupal-gebruikers veilig te authenticeren bij FastComments (aanbevolen).
- **Commenting Style** - Het type widget dat wordt weergegeven:
  - **Live Comments** - Real-time geneste reacties.
  - **Streaming Chat** - Live chat-interface.
  - **Collab Chat** - Collaboratieve annotatie via tekstselectie in het hoofdinhoudsgebied.
  - **Collab Chat + Comments** - Zowel collab chat als standaardreacties.
- **CDN URL** - FastComments CDN-URL (standaard: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments-site-URL (standaard: `https://fastcomments.com`).
- **Email notifications** - Stuur een e-mail naar de auteurs van de inhoud wanneer een nieuwe reactie op hun inhoud wordt geplaatst.

### Reacties toevoegen aan inhoudstypen

Voeg het **FastComments**-veld toe aan uw inhoudstypes via **Structuur > Inhoudstypes > [type] > Velden beheren**. Het veld heeft een status-schakelaar en een optionele aangepaste identificator per entiteit.

### EU Data Residency

Voor EU-gegevensresidentie, wijzig:
- **CDN URL** naar `https://cdn-eu.fastcomments.com`
- **Site URL** naar `https://eu.fastcomments.com`