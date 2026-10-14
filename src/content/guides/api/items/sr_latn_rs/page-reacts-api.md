Page Reacts omogućavaju vašim korisnicima da lajkuju stranicu ili reaguju na nju sa vašim sopstvenim skupom slika reakcija. The [Page Reacts widget](/guide-page-reacts.html) and the Floating Likes widget are built on these endpoints, and you can call them yourself to build your own like button.

Za razliku od ostatka ovog vodiča, Page Reacts endpoint‑i su javni. Pozivaju se iz pregledača vaših korisnika, ne zahtevaju API ključ i ne troše API kredite. Svaka reakcija pripada korisniku koji je poslao zahtev, tako da korisnik može da doda ili ukloni samo svoje reakcije.

Postoje dva skupa endpoint‑a:

- `/page-reacts/v1/likes/:tenantId` – jedan “like” po korisniku po stranici. Koristite ovo za dugme za lajkovanje.
- `/page-reacts/v2/:tenantId` – više reakcija po stranici, svaka identifikovana kratkim `id`‑om koji vi izaberete (na primer `heart` ili `laugh`).

Oba su takođe dostupna u našim SDK‑ovima kao deo `PublicApi`, na primer `getV1PageLikes`, `createV1PageReact` i `deleteV1PageReact` u [JavaScript SDK](/guide-sdk-javascript.html).

### Prikazivanje ko je reagovao

Prosledite `includeUserInfo=public` `GET` endpoint‑ima da biste takođe dobili korisnike koji su lajkovali ili reagovali, sa njihovim korisničkim imenima i avatarima, najnoviji prvi. Korisnici koji su postavili svoju aktivnost profila na privatnu su izostavljeni, kao i anonimni korisnici. Vraća se najviše 100 reakcija.

SSO korisnici se kreiraju sa privatnom aktivnošću profila osim ako vaš SSO payload ne postavi `isProfileActivityPrivate` na `false`, pa to podesite za korisnike koje želite da se pojave u ovim listama.

Na stranicama koje zahtevaju SSO, samo SSO korisnici koji mogu da vide stranicu mogu da prikažu ko je reagovao. Ostali dobijaju `403` sa kodom `sso-required` ili `page-forbidden`.

Sa vašeg servera, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) i [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) vraćaju iste liste uz API ključ, i takođe prihvataju `includeUserInfo=include-private` da uključe korisnike sa privatnim profilima.

### Identifikacija korisnika

Reakcije su vezane za korisnika koji šalje zahtev:

- **SSO korisnici:** prosledite `sso` query parametar, postavljen na URI‑enkodovan JSON istog SSO objekta koji dajete widget‑u za komentare. Pogledajte [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonimni korisnici:** kada ne postoji `sso` parametar i nema FastComments prijave, server dodeljuje pregledaču anonimni ID koji se čuva u FastComments sesijskom kolačiću. Šaljite zahteve sa `credentials: 'include'` kako bi kolačić bio zadržan između zahteva. Pregledači koji blokiraju kolačiće trećih strana neće zadržati anonimni ID, pa koristite SSO kada svaki korisnik mora da bude pouzdano prepoznat.

### urlId

`urlId` identifikuje stranicu, isto kao i za komentare. Koristite isti `urlId` koji dajete widget‑u za komentare kako bi lajkovi i komentari bili brojani na istoj stranici. Zapamtite da ga URI‑enkodujete.

[inline-code-attrs-start title = 'Primer dugmeta za lajkovanje'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
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