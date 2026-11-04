Alle instellingen bevinden zich onder `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Vereist

- **Tenant ID** - Je FastComments Tenant ID. Je vindt deze onder [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Vereist voor Secure SSO, webhook-verificatie en paginasynchronisatie. Te vinden onder [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Het API Secret opslaan

De module leest het API Secret uit de eerste van deze bronnen die een waarde bevat:

1. **Key module** - Installeer [Key](https://www.drupal.org/project/key), maak een *Authentication*-sleutel aan en selecteer deze onder **API Secret key** op het instellingenformulier. De sleutel kan worden ondersteund door een omgevingsvariabele of een bestand buiten de web root.
2. **settings.php** - Voeg `$config['fastcomments.settings']['api_secret'] = 'your-secret';` toe aan `settings.php` of `settings.local.php`. Het secret komt nooit in geëxporteerde configuratie terecht.
3. **Het instellingenformulier** - Vul het in het veld **API Secret** in. Dit slaat de waarde op in `fastcomments.settings`, waardoor deze in configuratie-exporten terechtkomt. Het Drupal-statusrapport geeft een waarschuwing wanneer dit de actieve bron is.

Het formulier toont welke bron actief is. Als je het veld leeg laat, blijft het opgeslagen secret behouden. Vink **Remove the API Secret stored in configuration** aan om het te verwijderen nadat je het secret naar de Key module of `settings.php` hebt verplaatst.

## Commentaarstijl

Kies de widget die past bij hoe je wilt dat mensen op je site met elkaar praten.

- **Live Comments** - Reacties in real-time met discussielijnen.
- **Streaming Chat** - Live chat-interface, geschikt voor evenementen en livestreams.
- **Collab Chat** - Annotatie op tekstselectie in het hoofdcontentgebied. Bezoekers markeren tekst en starten een discussie in context.
- **Collab Chat + Comments** - Zowel collab chat als standaardreacties op dezelfde pagina.

## SSO-modus

- **None** - Geen SSO. Gebruikers reageren als gasten of maken een FastComments-account aan.
- **Simple** - Geeft Drupal-gebruikersgegevens (naam, e-mail, avatar) door aan FastComments zonder server-side verificatie.
- **Secure** - Gebruikt HMAC-SHA256 om Drupal-gebruikers te verifiëren bij FastComments. Aanbevolen wanneer je een API Secret hebt geconfigureerd.

Zie de sectie `Single Sign-On (SSO)` voor meer details.

## Overige instellingen

- **CDN URL** - Standaard `https://cdn.fastcomments.com`.
- **Site URL** - Standaard `https://fastcomments.com`.
- **E-mailnotificaties** - Stuur een e-mail naar een contentauteur wanneer er een nieuwe reactie op hun content wordt geplaatst.

Zie voor EU-dataresidentie de sectie `EU Data Residency`.