Page Reacts omogočajo vašim uporabnikom, da všečajo stran ali se na njo odzovejo s svojim naborom slik reakcij. The [Page Reacts widget](/guide-page-reacts.html) and the Floating Likes widget are built on these endpoints, and you can call them yourself to build your own like button.

Za razliko od preostalega vodnika so končne točke Page Reacts javne. Kličete jih iz brskalnikov vaših uporabnikov, ne potrebujejo API ključa in ne porabijo API kreditov. Vsaka reakcija pripada uporabniku, ki je poslal zahtevo, zato lahko uporabnik doda ali odstrani le svoje reakcije.

Obstajata dva niza končnih točk:

- `/page-reacts/v1/likes/:tenantId` – en sam "like" na uporabnika na stran. Uporabite to za gumb všečkov.
- `/page-reacts/v2/:tenantId` – več reakcij na stran, vsaka identificirana s kratkim `id`, ki ga izberete (na primer `heart` ali `laugh`).

Obe sta na voljo tudi v naših SDK-jih kot del `PublicApi`, na primer `getV1PageLikes`, `createV1PageReact` in `deleteV1PageReact` v [JavaScript SDK](/guide-sdk-javascript.html).

### Identifikacija uporabnika

Reacts are tied to the user making the request:

- **SSO uporabniki:** pass the `sso` query parameter, set to the URI encoded JSON of the same SSO object you give the comment widget. See [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonimni uporabniki:** when there is no `sso` parameter and no FastComments login, the server assigns the browser an anonymous id stored in the FastComments session cookie. Send requests with `credentials: 'include'` so the cookie is kept between requests. Browsers that block third-party cookies will not keep the anonymous id, so use SSO when each user must be recognized reliably.

### urlId

`urlId` identifies the page, the same as it does for comments. Use the same `urlId` you give the comment widget so likes and comments are counted on the same page. Remember to URI encode it.

[inline-code-attrs-start title = 'Primer gumba všečkov'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opcijsko, za SSO uporabnike. Enak objekt, ki ga podate v možnost "sso" pripomočka za komentarje.
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