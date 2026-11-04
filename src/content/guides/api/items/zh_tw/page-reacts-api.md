頁面反應（Page Reacts）讓您的使用者可以按讚某個頁面，或使用您自訂的一組反應圖片來對頁面做出反應。[Page Reacts 小工具](/guide-page-reacts.html)與浮動按讚（Floating Likes）小工具都是基於這些端點建構的，您也可以自行呼叫它們來打造自己的按讚按鈕。

與本指南的其他部分不同，Page Reacts 端點是公開的。它們是從您的使用者瀏覽器呼叫的，不需要 API 金鑰，也不消耗 API 點數。每個反應都屬於發出請求的使用者，因此使用者只能新增或移除自己的反應。

有兩組端點：

- `/page-reacts/v1/likes/:tenantId` - 每位使用者對每個頁面僅能有一個「讚」。用於按讚按鈕。
- `/page-reacts/v2/:tenantId` - 每個頁面可有多個反應，每個反應由您選擇的簡短 `id` 識別（例如 `heart` 或 `laugh`）。

兩者也可透過我們 SDK 中的 `PublicApi` 使用，例如 [JavaScript SDK](/guide-sdk-javascript.html) 中的 `getV1PageLikes`、`createV1PageReact` 與 `deleteV1PageReact`。

### 顯示誰做出了反應

在 `GET` 端點傳入 `includeUserInfo=public`，即可同時取得按讚或做出反應的使用者，包含其使用者名稱與頭像，並按最近時間排序。已將個人資料活動設為私人的使用者會被排除，匿名使用者亦同。最多會回傳 100 筆反應。

SSO 使用者預設會以私人個人資料活動建立，除非您的 SSO 負載將 `isProfileActivityPrivate` 設為 `false`，因此請為您希望出現在這些清單中的使用者進行設定。

在需要 SSO 的頁面上，只有能檢視該頁面的 SSO 使用者才能列出誰做出了反應。其他人會收到帶有 `sso-required` 或 `page-forbidden` 代碼的 `403` 錯誤。

從您的伺服器端，[`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) 與 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) 可使用 API 金鑰回傳相同的清單，並且也接受 `includeUserInfo=include-private` 以包含具有私人個人資料的使用者。

### 識別使用者

反應會與發出請求的使用者綁定：

- **SSO 使用者：** 傳入 `sso` 查詢參數，其值為您提供給留言小工具之相同 SSO 物件的 URI 編碼 JSON。請參閱 [SSO](/guide-customizations-and-configuration.html#sso)。
- **匿名使用者：** 當沒有 `sso` 參數且沒有 FastComments 登入時，伺服器會為瀏覽器指派一個儲存在 FastComments 工作階段 Cookie 中的匿名 id。請使用 `credentials: 'include'` 發送請求，以確保 Cookie 在請求之間被保留。封鎖第三方 Cookie 的瀏覽器將無法保留匿名 id，因此在需要可靠識別每位使用者時，請使用 SSO。

### urlId

`urlId` 用於識別頁面，與留言功能中的用法相同。請使用與您提供給留言小工具相同的 `urlId`，這樣按讚與留言才會計入同一個頁面。記得要進行 URI 編碼。

[inline-code-attrs-start title = '按讚按鈕範例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// 選填，適用於 SSO 使用者。與您提供給留言小工具 "sso" 選項的相同物件。
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