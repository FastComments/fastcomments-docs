Page Reacts laten uw gebruikers een pagina leuk vinden, of erop reageren met uw eigen set reactievormafbeeldingen. De [Page Reacts widget](/guide-page-reacts.html) en de Floating Likes widget zijn gebouwd op deze eindpunten, en u kunt ze zelf aanroepen om uw eigen like‑knop te bouwen.

In tegenstelling tot de rest van deze gids, zijn de Page Reacts‑eindpunten openbaar. Ze worden aangeroepen vanuit de browsers van uw gebruikers, vereisen geen API‑sleutel en kosten geen API‑credits. Elke reactie behoort tot de gebruiker die het verzoek doet, dus een gebruiker kan alleen zijn eigen reacties toevoegen of verwijderen.

Er zijn twee sets eindpunten:

- `/page-reacts/v1/likes/:tenantId` – één enkele “like” per gebruiker per pagina. Gebruik deze voor een like‑knop.
- `/page-reacts/v2/:tenantId` – meerdere reacties per pagina, elk geïdentificeerd door een korte `id` die u kiest (bijvoorbeeld `heart` of `laugh`).

Beide zijn ook beschikbaar in onze SDK’s als onderdeel van de `PublicApi`, bijvoorbeeld `getV1PageLikes`, `createV1PageReact` en `deleteV1PageReact` in de [JavaScript SDK](/guide-sdk-javascript.html).

### De gebruiker identificeren

Reacties zijn gekoppeld aan de gebruiker die het verzoek doet:

- **SSO‑gebruikers:** geef de `sso`‑queryparameter mee, ingesteld op de URI‑gecodeerde JSON van hetzelfde SSO‑object dat u aan de commentaarwidget geeft. Zie [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonieme gebruikers:** wanneer er geen `sso`‑parameter is en geen FastComments‑login, kent de server de browser een anonieme id toe die wordt opgeslagen in de FastComments‑sessiecookie. Verstuur verzoeken met `credentials: 'include'` zodat de cookie tussen verzoeken behouden blijft. Browsers die third‑party cookies blokkeren, behouden de anonieme id niet, dus gebruik SSO wanneer elke gebruiker betrouwbaar herkend moet worden.

### De urlId

`urlId` identificeert de pagina, hetzelfde als voor reacties. Gebruik dezelfde `urlId` die u aan de commentaarwidget geeft zodat likes en reacties op dezelfde pagina worden geteld. Vergeet niet om deze URI‑gecodeerd te gebruiken.

[inline-code-attrs-start title = 'Voorbeeld van Like-knop'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optioneel, voor SSO‑gebruikers. Hetzelfde object dat u aan de commentaarwidget's "sso" optie geeft.
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

---