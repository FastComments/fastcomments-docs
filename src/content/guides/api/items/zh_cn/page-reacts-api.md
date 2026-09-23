Page Reacts 让您的用户可以点赞页面，或使用您自定义的一组表情图片进行回应。The [Page Reacts widget](/guide-page-reacts.html) 和 Floating Likes 小部件基于这些端点构建，您也可以自行调用它们来创建自己的点赞按钮。

与本指南的其他部分不同，Page Reacts 的端点是公开的。它们在用户的浏览器中调用，不需要 API 密钥，也不消耗 API 积分。每一次回应都归属于发起请求的用户，因此用户只能添加或移除自己的回应。

有两套端点：

- `/page-reacts/v1/likes/:tenantId` - 每个用户在每个页面只能有一个“点赞”。用于点赞按钮。
- `/page-reacts/v2/:tenantId` - 每个页面可以有多个回应，每个由您自定义的短 `id` 标识（例如 `heart` 或 `laugh`）。

这两个端点也在我们的 SDK 中作为 `PublicApi` 的一部分提供，例如在 [JavaScript SDK](/guide-sdk-javascript.html) 中的 `getV1PageLikes`、`createV1PageReact` 和 `deleteV1PageReact`。

### 识别用户

Reacts are tied to the user making the request:

- **SSO 用户：** 传递 `sso` 查询参数，其值为与您提供给评论小部件的相同 SSO 对象的 URI 编码 JSON。参见 [SSO](/guide-customizations-and-configuration.html#sso)。
- **匿名用户：** 当没有 `sso` 参数且未登录 FastComments 时，服务器会为浏览器分配一个存储在 FastComments 会话 cookie 中的匿名 ID。发送请求时使用 `credentials: 'include'`，以便在请求之间保留 cookie。阻止第三方 cookie 的浏览器将无法保留匿名 ID，因此在需要可靠识别每个用户时请使用 SSO。

### urlId

`urlId` 用于标识页面，与评论使用的方式相同。请使用您提供给评论小部件的相同 `urlId`，这样点赞和评论会在同一页面上统计。记得对其进行 URI 编码。

[inline-code-attrs-start title = '点赞按钮示例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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