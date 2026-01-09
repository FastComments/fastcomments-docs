[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO 使用 HMAC-SHA256 加密作为实现 SSO 的机制。首先我们将介绍总体架构，提供示例和详细步骤。

还有一些关于从具有类似 SSO 机制的其他提供商迁移的文档，以及它们之间的差异。

流程如下所示：

<div class="screenshot white-bg">
    <div class="title">安全 SSO 流程</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="安全 SSO 图示" />
</div>

由于 Secure SSO 涉及全栈开发，目前 Java/Spring、NodeJS/Express 和 原生 PHP 的完整可运行代码示例已在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub</a> 上。

尽管我们在 NodeJS 示例中使用 ExpressJS，在 Java 示例中使用 Spring，但在这些运行时中实现 FastComments SSO 并不需要任何框架/库——使用原生的 crypto 包即可。

使用 FastComments SSO 无需编写任何新的 API 端点。只需使用您的密钥对用户信息进行加密，然后将负载传递给评论组件。

#### Get Your API Secret Key

您的 API Secret 可以从 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">此页面</a> 获取。您也可以通过进入 My Account，点击 API/SSO 磁贴，然后点击 “Get API Secret Key” 来找到此页面。

#### Comment Widget Parameters

评论组件的高级 API 文档可在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">此处</a> 找到。

下面我们详细说明这些参数的含义。

评论组件接受一个配置对象——如果您使用 FastComments，通常已经传入用于标识客户的 tenantId。

要启用 SSO，传入一个新的 "sso" 对象，该对象必须包含以下参数。这些值应在服务器端生成。

- userDataJSONBase64: 用户的 JSON 格式数据，然后进行 Base64 编码。
- verificationHash: 由 UNIX_TIME_MILLIS + userDataJSONBase64 创建的 HMAC-SHA256 哈希值。
- timestamp: Epoch 时间戳，单位为 **毫秒**。不能是未来的时间，也不能早于两天以前。
- loginURL: 评论组件可用于展示的登录 URL。
- logoutURL: 评论组件可用于展示的登出 URL。
- loginCallback: 如果提供了该函数（而非 login URL），评论组件在点击登录按钮时会调用此函数。
- logoutCallback: 如果提供了该函数（而非 logout URL），评论组件在点击登出按钮时会调用此函数。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### 用户对象

[inline-code-attrs-start title = '用户对象'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必需。最多 1k 字符。 **/
    id: string;
    /** 必需。最多 1k 字符。注意：必须唯一。 **/
    email: string;
    /** 必需。最多 1k 字符。注意：用户名不能是电子邮件。无需唯一。 **/
    username: string;
    /** 可选。URL 最多 3k 字符。默认基于邮箱从 gravatar 获取。支持 64 编码的图像，在这种情况下限制为 50k 字符。 **/ 
    avatar?: string;
    /** 可选。默认 false。 **/
    optedInNotifications?: boolean;
    /** 可选。默认 false。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 可选。最多 100 字符。该标签将显示在其姓名旁。适用时默认是 Administrator/Moderator。 **/
    displayLabel?: string;
    /** 可选。最多 500 字符。这将替代用户名显示。 **/
    displayName?: string;
    /** 可选。最多 2k 字符。用户的姓名将链接到此地址。 **/
    websiteUrl?: string;
    /** 可选。每个用户最多 100 个组。组 id 不能超过 50 字符。 **/
    groupIds?: string[];
    /** 可选。表示该用户为管理员。 **/
    isAdmin?: boolean;
    /** 可选。表示该用户为版主。 **/
    isModerator?: boolean;
    /** 可选，默认 true。设置为 false 可在用户资料中启用“活动”选项卡。 **/
    isProfileActivityPrivate?: boolean;
    /** 可选，默认 false。设置为 true 可禁用资料评论。 **/
    isProfileCommentsPrivate?: boolean;
    /** 可选，默认 false。设置为 true 可禁用向该用户发送私信。 **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

对于管理员和版主，请在 `SSOUser` 对象中传入相应的 `isAdmin` 或 `isModerator` 标志。

#### Notifications

要启用或禁用通知，请将 `optedInNotifications` 的值分别设置为 `true` 或 `false`。当用户首次在 SSO 负载中带有此值加载页面时，他们的通知设置将被更新。

另外，如果您希望用户针对他们订阅的页面上的活动收到邮件通知（而不仅仅是应用内通知），请将 `optedInSubscriptionNotifications` 设置为 `true`。

#### VIP Users & Special Labels

您可以使用可选字段 "displayLabel" 在用户姓名旁显示特殊标签。

#### Unauthenticated users

要表示未认证用户，只需不填充 userDataJSONBase64、verificationHash 或 timestamp，并提供一个 loginURL。

这些用户将无法发表评论，而是会看到登录提示（消息、链接或按钮，取决于配置）。

#### Direct Examples for Serializing and Hashing User Data

更多细节示例见 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">这里</a>（js）、<a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">这里</a>（java）和 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">这里</a>（php）。

我们理解任何集成都可能是复杂且困难的过程。如有疑问，请随时联系您的客户代表或使用 <a href="https://fastcomments.com/auth/my-account/help" target="_blank">支持页面</a>。

---