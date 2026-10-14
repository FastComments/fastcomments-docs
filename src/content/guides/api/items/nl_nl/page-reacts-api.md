Page Reacts laat je gebruikers een pagina leuk vinden, of erop reageren met je eigen set reactievormafbeeldingen. De [Page Reacts widget](/guide-page-reacts.html) en de Floating Likes widget zijn gebouwd op deze eindpunten, en je kunt ze zelf aanroepen om je eigen like‑knop te bouwen.

In tegenstelling tot de rest van deze gids, zijn de Page Reacts‑eindpunten openbaar. Ze worden aangeroepen vanuit de browsers van je gebruikers, vereisen geen API‑sleutel en kosten geen API‑credits. Elke reactie behoort tot de gebruiker die het verzoek doet, dus een gebruiker kan alleen zijn eigen reacties toevoegen of verwijderen.

Er zijn twee sets eindpunten:

- `/page-reacts/v1/likes/:tenantId` – één enkele “like” per gebruiker per pagina. Gebruik deze voor een like‑knop.  
- `/page-reacts/v2/:tenantId` – meerdere reacties per pagina, elk geïdentificeerd door een korte `id` die je kiest (bijvoorbeeld `heart` of `laugh`).

Beide zijn ook beschikbaar in onze SDK’s als onderdeel van de `PublicApi`, bijvoorbeeld `getV1PageLikes`, `createV1PageReact` en `deleteV1PageReact` in de [JavaScript SDK](/guide-sdk-javascript.html).

### Tonen wie heeft gereageerd

Geef `includeUserInfo=public` mee aan de `GET`‑eindpunten om ook de gebruikers te krijgen die hebben geliked of gereageerd, met hun gebruikersnamen en avatars, meest recent eerst. Gebruikers die hun profielactiviteit privé hebben gemaakt, worden weggelaten, net als anonieme gebruikers. Tot 100 reacties worden geretourneerd.

SSO‑gebruikers worden aangemaakt met privé‑profielactiviteit tenzij je SSO‑payload `isProfileActivityPrivate` op `false` zet, dus stel het in voor de gebruikers die je in deze lijsten wilt laten verschijnen.

Op pagina's die SSO vereisen, kunnen alleen SSO‑gebruikers die de pagina kunnen bekijken zien wie heeft gereageerd. Anderen krijgen een `403` met de code `sso-required` of `page-forbidden`.

Van je server, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) en [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) retourneren dezelfde lijsten met een API‑sleutel, en accepteren ook `includeUserInfo=include-private` om gebruikers met privé‑profielen op te nemen.

### Identificeren van de gebruiker

Reacties zijn gekoppeld aan de gebruiker die het verzoek doet:

- **SSO‑gebruikers:** geef de `sso`‑queryparameter mee, ingesteld op de URI‑gecodeerde JSON van hetzelfde SSO‑object dat je aan de commentaarwidget geeft. Zie [SSO](/guide-customizations-and-configuration.html#sso).  
- **Anonieme gebruikers:** wanneer er geen `sso`‑parameter is en er geen FastComments‑login is, kent de server de browser een anonieme id toe die wordt opgeslagen in de FastComments‑sessiecookie. Verstuur verzoeken met `credentials: 'include'` zodat de cookie tussen verzoeken behouden blijft. Browsers die third‑party cookies blokkeren, behouden de anonieme id niet, dus gebruik SSO wanneer elke gebruiker betrouwbaar herkend moet worden.

### De urlId

`urlId` identificeert de pagina, hetzelfde als voor reacties. Gebruik dezelfde `urlId` die je aan de commentaarwidget geeft zodat likes en reacties op dezelfde pagina worden geteld. Vergeet niet om deze te URI‑encoderen.

[inline-code-attrs-start title = 'Voorbeeld van Like‑knop'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optioneel, voor SSO‑gebruikers. Hetzelfde object dat je aan de commentaarwidget's "sso"‑optie geeft.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]