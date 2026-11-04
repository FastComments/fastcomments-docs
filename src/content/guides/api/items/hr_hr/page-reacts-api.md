Page Reacts omogućuju vašim korisnicima da lajkaju stranicu ili na nju reagiraju vlastitim skupom slika reakcija. [Page Reacts widget](/guide-page-reacts.html) i Floating Likes widget izgrađeni su na ovim endpointima, a možete ih i sami pozivati kako biste izgradili vlastitu lajk tipku.

Za razliku od ostatka ovog vodiča, Page Reacts endpointi su javni. Pozivaju se iz preglednika vaših korisnika, ne zahtijevaju API ključ i ne koštaju API kredita. Svaka reakcija pripada korisniku koji upućuje zahtjev, pa korisnik može dodati ili ukloniti samo svoju.

Postoje dva skupa endpointa:

- `/page-reacts/v1/likes/:tenantId` - jedan "lajk" po korisniku po stranici. Koristite ovo za lajk tipku.
- `/page-reacts/v2/:tenantId` - više reakcija po stranici, svaka identificirana kratkim `id`-om koji sami odaberete (na primjer `heart` ili `laugh`).

Oboje su dostupni i u našim SDK-ovima kao dio `PublicApi`, na primjer `getV1PageLikes`, `createV1PageReact` i `deleteV1PageReact` u [JavaScript SDK-u](/guide-sdk-javascript.html).

### Prikaz tko je reagirao

Proslijedite `includeUserInfo=public` na `GET` endpointe kako biste dobili i korisnike koji su lajkali ili reagirali, s njihovim korisničkim imenima i avatarima, najnoviji prvi. Korisnici koji su svoju aktivnost profila učinili privatnom su isključeni, kao i anonimni korisnici. Vraća se do 100 reakcija.

SSO korisnici se stvaraju s privatnom aktivnošću profila osim ako vaš SSO payload ne postavi `isProfileActivityPrivate` na `false`, pa to postavite za korisnike koje želite vidjeti u ovim listama.

Na stranicama koje zahtijevaju SSO, samo SSO korisnici koji mogu vidjeti stranicu mogu prikazati tko je reagirao. Ostali dobivaju `403` s kodom `sso-required` ili `page-forbidden`.

Sa svog poslužitelja, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) i [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) vraćaju iste liste s API ključem, a također prihvaćaju `includeUserInfo=include-private` kako bi uključili korisnike s privatnim profilima.

### Identifikacija korisnika

Reakcije su vezane uz korisnika koji upućuje zahtjev:

- **SSO korisnici:** proslijedite `sso` query parametar, postavljen na URI kodirani JSON istog SSO objekta koji dajete comment widgetu. Pogledajte [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonimni korisnici:** kada nema `sso` parametra ni FastComments prijave, poslužitelj pregledniku dodjeljuje anonimni id pohranjen u FastComments session cookieju. Šaljite zahtjeve s `credentials: 'include'` kako bi cookie ostao između zahtjeva. Preglednici koji blokiraju third-party cookieje neće zadržati anonimni id, pa koristite SSO kada svaki korisnik mora pouzdano biti prepoznat.

### urlId

`urlId` identificira stranicu, isto kao i za komentare. Koristite isti `urlId` koji dajete comment widgetu kako bi se lajkovi i komentari brojali na istoj stranici. Ne zaboravite ga URI kodirati.

[inline-code-attrs-start title = 'Primjer lajk tipke'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opcionalno, za SSO korisnike. Isti objekt koji dajete "sso" opciji comment widgeta.
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