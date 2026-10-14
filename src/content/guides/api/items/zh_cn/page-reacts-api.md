Page Reacts 让您的用户可以点赞页面，或使用您自定义的一组表情图片进行反应。The [Page Reacts widget](/guide-page-reacts.html) 和 Floating Likes 小部件基于这些端点构建，您也可以自行调用它们来构建自己的点赞按钮。

与本指南的其余部分不同，Page Reacts 端点是公开的。它们从用户的浏览器中调用，不需要 API 密钥，也不消耗 API 积分。每个反应都归属于发起请求的用户，因此用户只能添加或移除自己的反应。

有两组端点：

- `/page-reacts/v1/likes/:tenantId` - 每个用户每页仅有一个 “like”。用于点赞按钮。
- `/page-reacts/v2/:tenantId` - 每页可有多个反应，每个反应由您选择的短 `id` 标识（例如 `heart` 或 `laugh`）。

这两组端点也在我们的 SDK 中作为 `PublicApi` 的一部分提供，例如在 [JavaScript SDK](/guide-sdk-javascript.html) 中的 `getV1PageLikes`、`createV1PageReact` 和 `deleteV1PageReact`。

### Showing Who Reacted

在 `GET` 端点上添加 `includeUserInfo=public` 参数，可同时获取点赞或反应的用户信息，包括用户名和头像，按最近的先后顺序返回。将个人资料活动设为私密的用户会被排除，匿名用户也会被排除。最多返回 100 条反应。

SSO 用户默认创建为私密的个人资料活动，除非您的 SSO 负载将 `isProfileActivityPrivate` 设置为 `false`，因此请为您希望出现在这些列表中的用户进行相应设置。

在需要 SSO 的页面上，只有能够查看该页面的 SSO 用户才能列出谁进行了反应。其他用户会收到 `403` 错误，错误码为 `sso-required` 或 `page-forbidden`。

从您的服务器端，[`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) 和 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) 在使用 API 密钥时返回相同的列表，并且同样接受 `includeUserInfo=include-private` 参数，以包含个人资料设为私密的用户。

### Identifying the User

反应与发起请求的用户绑定：

- **SSO 用户：** 传递 `sso` 查询参数，值为与评论小部件相同的 SSO 对象的 URI 编码 JSON。参见 [SSO](/guide-customizations-and-configuration.html#sso)。
- **匿名用户：** 当没有 `sso` 参数且没有 FastComments 登录时，服务器会为浏览器分配一个存储在 FastComments 会话 cookie 中的匿名 ID。发送请求时使用 `credentials: 'include'`，以便在请求之间保留 cookie。阻止第三方 cookie 的浏览器将无法保留匿名 ID，因此在需要可靠识别每个用户的情况下请使用 SSO。

### The urlId

`urlId` 用于标识页面，与评论使用的方式相同。使用与评论小部件相同的 `urlId`，这样点赞和评论会计入同一页面。记得对其进行 URI 编码。

[inline-code-attrs-start title = '点赞按钮示例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// 可选，针对 SSO 用户。与您提供给评论小部件的 "sso" 选项相同的对象。
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