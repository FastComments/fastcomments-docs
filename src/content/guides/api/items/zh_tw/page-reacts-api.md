Page Reacts 讓您的使用者可以按讚頁面，或使用您自訂的一組回應圖示來回應。 [Page Reacts 小工具](/guide-page-reacts.html) 與 Floating Likes 小工具是基於這些端點建置的，您也可以自行呼叫它們來建立自己的按讚按鈕。

與本指南的其他部分不同，Page Reacts 端點是公開的。它們從使用者的瀏覽器呼叫，不需要 API 金鑰，也不會消耗 API 點數。每個回應都屬於發出請求的使用者，因此使用者只能新增或移除自己的回應。

有兩組端點：

- `/page-reacts/v1/likes/:tenantId` - 每位使用者每頁僅有一個「讚」。用於按讚按鈕。
- `/page-reacts/v2/:tenantId` - 每頁可有多個回應，每個回應以您自行選擇的短 `id`（例如 `heart` 或 `laugh`）識別。

兩者也都在我們的 SDK 中作為 `PublicApi` 的一部份提供，例如在 [JavaScript SDK](/guide-sdk-javascript.html) 中的 `getV1PageLikes`、`createV1PageReact` 與 `deleteV1PageReact`。

### Identifying the User

回應與發出請求的使用者相關聯：

- **SSO 使用者：** 傳遞 `sso` 查詢參數，設定為與您提供給評論小工具的相同 SSO 物件的 URI 編碼 JSON。請參閱 [SSO](/guide-customizations-and-configuration.html#sso)。
- **匿名使用者：** 當沒有 `sso` 參數且未登入 FastComments 時，伺服器會為瀏覽器指派一個儲存在 FastComments 會話 Cookie 中的匿名 ID。請以 `credentials: 'include'` 送出請求，以便在請求之間保留 Cookie。阻擋第三方 Cookie 的瀏覽器將不會保留匿名 ID，若每位使用者必須可靠辨識，請使用 SSO。

### The urlId

`urlId` 用於識別頁面，與評論的識別方式相同。請使用您提供給評論小工具的相同 `urlId`，以便在同一頁面上同時計算讚與評論。記得對其進行 URI 編碼。

[inline-code-attrs-start title = '喜歡按鈕範例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// 可選，供 SSO 使用者使用。與您提供給評論小工具的 "sso" 選項相同的物件。
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