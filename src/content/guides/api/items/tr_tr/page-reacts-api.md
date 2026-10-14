Page Reacts, kullanıcılarınızın bir sayfayı beğenmesine veya kendi reaksiyon görselleri setinizle ona tepki vermesine olanak tanır. The [Page Reacts widget](/guide-page-reacts.html) and the Floating Likes widget are built on these endpoints, and you can call them yourself to build your own like button.

Unlike the rest of this guide, the Page Reacts endpoints are public. They are called from your users' browsers, take no API key, and cost no API credits. Every react belongs to the user making the request, so a user can only add or remove their own.

There are two sets of endpoints:

- `/page-reacts/v1/likes/:tenantId` - bir sayfa başına kullanıcı başına tek bir "like". Use these for a like button.
- `/page-reacts/v2/:tenantId` - bir sayfa başına birden fazla reaksiyon, her biri sizin seçtiğiniz kısa bir `id` ile tanımlanır (örneğin `heart` veya `laugh`).

Both are also available in our SDKs as part of the `PublicApi`, for example `getV1PageLikes`, `createV1PageReact`, and `deleteV1PageReact` in the [JavaScript SDK](/guide-sdk-javascript.html).

### Showing Who Reacted

Pass `includeUserInfo=public` to the `GET` endpoints to also get the users who liked or reacted, with their usernames and avatars, most recent first. Users who have made their profile activity private are left out, as are anonymous users. Up to 100 reacts are returned.

SSO users are created with private profile activity unless your SSO payload sets `isProfileActivityPrivate` to `false`, so set it for the users you want to appear in these lists.

On pages that require SSO, only SSO users who can view the page can list who reacted. Others get a `403` with the code `sso-required` or `page-forbidden`.

From your server, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) and [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) return the same lists with an API key, and also accept `includeUserInfo=include-private` to include users with private profiles.

### Identifying the User

Reacts are tied to the user making the request:

- **SSO users:** pass the `sso` query parameter, set to the URI encoded JSON of the same SSO object you give the comment widget. See [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonymous users:** when there is no `sso` parameter and no FastComments login, the server assigns the browser an anonymous id stored in the FastComments session cookie. Send requests with `credentials: 'include'` so the cookie is kept between requests. Browsers that block third-party cookies will not keep the anonymous id, so use SSO when each user must be recognized reliably.

### The urlId

`urlId` identifies the page, the same as it does for comments. Use the same `urlId` you give the comment widget so likes and comments are counted on the same page. Remember to URI encode it.

[inline-code-attrs-start title = 'Beğen Düğmesi Örneği'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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