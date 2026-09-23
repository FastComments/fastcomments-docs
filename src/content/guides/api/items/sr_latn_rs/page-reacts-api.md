Page Reacts omogućavaju vašim korisnicima da lajkuju stranicu ili reaguju na nju pomoću vašeg sopstvenog skupa slika reakcija. The [Page Reacts widget](/guide-page-reacts.html) and the Floating Likes widget are built on these endpoints, and you can call them yourself to build your own like button.

Za razliku od ostatka ovog vodiča, Page Reacts endpoint-i su javni. They are called from your users' browsers, take no API key, and cost no API credits. Every react belongs to the user making the request, so a user can only add or remove their own.

There are two sets of endpoints:

- `/page-reacts/v1/likes/:tenantId` - jedan „like“ po korisniku po stranici. Use these for a like button.
- `/page-reacts/v2/:tenantId` - više reakcija po stranici, svaka identifikovana kratkim `id`‑om koji izaberete (na primer `heart` ili `laugh`).

Both are also available in our SDKs as part of the `PublicApi`, for example `getV1PageLikes`, `createV1PageReact`, and `deleteV1PageReact` in the [JavaScript SDK](/guide-sdk-javascript.html).

### Identifikacija korisnika

Reacts are tied to the user making the request:

- **SSO korisnici:** pass the `sso` query parameter, set to the URI encoded JSON of the same SSO object you give the comment widget. See [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonimni korisnici:** when there is no `sso` parameter and no FastComments login, the server assigns the browser an anonymous id stored in the FastComments session cookie. Send requests with `credentials: 'include'` so the cookie is kept between requests. Browsers that block third-party cookies will not keep the anonymous id, so use SSO when each user must be recognized reliably.

### urlId

`urlId` identifies the page, the same as it does for comments. Use the same `urlId` you give the comment widget so likes and comments are counted on the same page. Remember to URI encode it.

[inline-code-attrs-start title = 'Primer dugmeta za lajk'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opcionalno, za SSO korisnike. Isti objekat koji prosleđujete opciji "sso" widgeta za komentare.
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