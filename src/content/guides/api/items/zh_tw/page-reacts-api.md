Page Reacts 讓您的使用者可以為頁面按讚，或使用您自訂的一組回應圖示來回應。 The [Page Reacts widget](/guide-page-reacts.html) 與 Floating Likes 小工具是基於這些端點建構的，您也可以自行呼叫它們來建立自己的按讚按鈕。

與本指南的其他部分不同，Page Reacts 端點是公開的。它們由使用者的瀏覽器呼叫，不需要 API 金鑰，也不會消耗 API 點數。每個回應都屬於發出請求的使用者，因此使用者只能新增或移除自己的回應。

There are two sets of endpoints:

- `/page-reacts/v1/likes/:tenantId` - 每位使用者在每個頁面只能有一次「讚」。可用於按讚按鈕。
- `/page-reacts/v2/:tenantId` - 每個頁面可有多個回應，每個回應以您自行選擇的短 `id` 來識別（例如 `heart` 或 `laugh`）。

Both are also available in our SDKs as part of the `PublicApi`, for example `getV1PageLikes`, `createV1PageReact`, and `deleteV1PageReact` in the [JavaScript SDK](/guide-sdk-javascript.html).

### 顯示誰回應了

Pass `includeUserInfo=public` to the `GET` endpoints to also get the users who liked or reacted, with their usernames and avatars, most recent first. Users who have made their profile activity private are left out, as are anonymous users. Up to 100 reacts are returned.

SSO users are created with private profile activity unless your SSO payload sets `isProfileActivityPrivate` to `false`, so set it for the users you want to appear in these lists.

On pages that require SSO, only SSO users who can view the page can list who reacted. Others get a `403` with the code `sso-required` or `page-forbidden`.

From your server, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) and [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) return the same lists with an API key, and also accept `includeUserInfo=include-private` to include users with private profiles.

### 識別使用者

Reacts are tied to the user making the request:

- **SSO 使用者：** pass the `sso` query parameter, set to the URI encoded JSON of the same SSO object you give the comment widget. See [SSO](/guide-customizations-and-configuration.html#sso).
- **匿名使用者：** when there is no `sso` parameter and no FastComments login, the server assigns the browser an anonymous id stored in the FastComments session cookie. Send requests with `credentials: 'include'` so the cookie is kept between requests. Browsers that block third-party cookies will not keep the anonymous id, so use SSO when each user must be recognized reliably.

### urlId

`urlId` 用於識別頁面，與評論的識別方式相同。請使用您提供給評論小工具的相同 `urlId`，以確保按讚與評論計算在同一頁面上。記得對其進行 URI 編碼。

[inline-code-attrs-start title = '喜歡按鈕範例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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