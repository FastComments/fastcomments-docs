页面 Reacts 让你的用户可以点赞页面，或使用你自定义的一组表情图片对页面做出反应。[Page Reacts widget](/guide-page-reacts.html) 和 Floating Likes widget 都基于这些端点构建，你也可以自己调用它们来构建自己的点赞按钮。

与本指南的其他部分不同，Page Reacts 端点是公开的。它们从用户的浏览器中调用，不需要 API key，也不消耗 API credits。每个反应都属于发起请求的用户，因此用户只能添加或删除自己的反应。

有两组端点：

- `/page-reacts/v1/likes/:tenantId` - 每个用户对每个页面只能有一个"赞"。用于点赞按钮。
- `/page-reacts/v2/:tenantId` - 每个页面可以有多个反应，每个反应由你选择的一个短 `id` 标识（例如 `heart` 或 `laugh`）。

两者也作为 `PublicApi` 的一部分在我们的 SDK 中提供，例如 [JavaScript SDK](/guide-sdk-javascript.html) 中的 `getV1PageLikes`、`createV1PageReact` 和 `deleteV1PageReact`。

### 显示谁做出了反应

向 `GET` 端点传递 `includeUserInfo=public`，即可同时获取点赞或做出反应的用户，包括他们的用户名和头像，按最近优先排列。已将个人资料活动设为私有的用户会被排除在外，匿名用户也是如此。最多返回 100 条反应。

SSO 用户默认创建为私有个人资料活动，除非你的 SSO payload 将 `isProfileActivityPrivate` 设置为 `false`，因此请为希望出现在这些列表中的用户设置此项。

在需要 SSO 的页面上，只有能够查看该页面的 SSO 用户才能列出谁做出了反应。其他用户会收到带有 `sso-required` 或 `page-forbidden` 代码的 `403` 错误。

从你的服务器端，[`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) 和 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) 使用 API key 返回相同的列表，并且还接受 `includeUserInfo=include-private` 以包含拥有私有个人资料的用户。

### 识别用户

反应与发起请求的用户绑定：

- **SSO 用户：** 传递 `sso` 查询参数，设置为与提供给评论 widget 的相同 SSO 对象的 URI 编码 JSON。参见 [SSO](/guide-customizations-and-configuration.html#sso)。
- **匿名用户：** 当没有 `sso` 参数且没有 FastComments 登录时，服务器会为浏览器分配一个匿名 id，该 id 存储在 FastComments 会话 cookie 中。请使用 `credentials: 'include'` 发送请求，以便在请求之间保留 cookie。阻止第三方 cookie 的浏览器将无法保留匿名 id，因此当需要可靠地识别每个用户时，请使用 SSO。

### urlId

`urlId` 标识页面，与评论的用法相同。使用与提供给评论 widget 相同的 `urlId`，这样点赞和评论就会统计在同一个页面上。记得对其进行 URI 编码。

[inline-code-attrs-start title = '点赞按钮示例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// 可选，用于 SSO 用户。与提供给评论 widget 的 "sso" 选项相同的对象。
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