[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO 使用 HMAC-SHA256 加密作为实现 SSO 的机制。首先我们将介绍整体架构、提供示例以及详细步骤。

也有一些关于从其他具有类似 SSO 机制的提供商迁移以及差异的文档。

流程如下所示：

<div class="screenshot white-bg">
    <div class="title">安全 SSO 流程</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="安全 SSO 图表" />
</div>

由于 Secure SSO 涉及全栈开发，完整可运行的 Java/Spring、NodeJS/Express 和 原生 PHP 的示例代码当前位于 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub 上</a>。

尽管我们在 NodeJS 示例中使用 ExpressJS，在 Java 示例中使用 Spring，但在这些运行时中实现 FastComments SSO 并不需要任何框架/库 —— 原生的加密包就可以工作。

使用 FastComments SSO 不需要编写任何新的 API 端点。只需使用您的密钥对用户信息进行加密并将负载传递给评论小组件即可。

#### 获取您的 API Secret Key

您的 API Secret 可以从 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">此页面</a> 获取。您也可以通过转到“我的账户”，点击 API/SSO 磁贴，然后点击“Get API Secret Key”来找到此页面。

#### 评论小组件参数

评论小组件的高级 API 文档可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">此处</a> 找到。

下面我们详细说明这些参数的含义。

评论小组件接收一个配置对象 —— 如果您正在使用 FastComments，您已经传递了该对象以传递您的客户 ID（称为 tenantId）。

要启用 SSO，请传入一个新的 "sso" 对象，该对象必须具有以下参数。其值应在服务器端生成。

- userDataJSONBase64: 用户的数据，JSON 格式，然后进行 Base64 编码。
- verificationHash: 由 UNIX_TIME_MILLIS + userDataJSONBase64 创建的 HMAC-SHA256 哈希。
- timestamp: 纪元时间戳，单位为 **毫秒**。不得为将来时间，且不能早于两天之前。
- loginURL: 评论小组件可用于让用户登录的 URL。
- logoutURL: 评论小组件可用于让用户登出的 URL。
- loginCallback: 如果提供此项而不是 login URL，评论小组件在点击登录按钮时会调用的函数。
- logoutCallback: 如果提供此项而不是 logout URL，评论小组件在点击登出按钮时会调用的函数。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### 用户对象

[inline-code-attrs-start title = '用户对象'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必需。最多 1k 字符。 **/
    id: string;
    /** 必需。最多 1k 字符。注意：必须唯一。 **/
    email: string;
    /** 必需。最多 1k 字符。注意：用户名不能是电子邮件。不必唯一。 **/
    username: string;
    /** 可选。URL 最多 3k 字符。默认基于 email 使用 gravatar。支持 base64 编码的图片，在这种情况下限制为 50k 字符。 **/ 
    avatar?: string;
    /** 可选。默认 false。 **/
    optedInNotifications?: boolean;
    /** 可选。默认 false。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 可选。最多 100 字符。此标签将显示在其名称旁边。适用时默认是 Administrator/Moderator。 **/
    displayLabel?: string;
    /** 可选。最多 500 字符。将替代用户名显示。 **/
    displayName?: string;
    /** 可选。最多 2k 字符。用户的名称将链接到此处。 **/
    websiteUrl?: string;
    /** 可选。每个用户最多 100 个组。组 id 不得超过 50 个字符。 **/
    groupIds?: string[];
    /** 可选。表示该用户为管理员。 **/
    isAdmin?: boolean;
    /** 可选。表示该用户为版主。 **/
    isModerator?: boolean;
    /** 可选，默认 true。设置为 false 以启用用户资料中的“activity”选项卡。 **/
    isProfileActivityPrivate?: boolean;
    /** 可选，默认 false。设置为 true 可禁用资料评论。 **/
    isProfileCommentsPrivate?: boolean;
    /** 可选，默认 false。设置为 true 可禁用向该用户发送直接消息。 **/
    isProfileDMDisabled?: boolean;
    /** 用户徽章的可选配置。 **/
    badgeConfig?: {
        /** 要分配的一组全局徽章 ID。限制为 30 个徽章。顺序保留。 **/
        badgeIds: string[];
        /** 针对当前页面（urlId）范围的徽章 ID 数组。仅在分配的页面上显示。 **/
        pageBadgeIds?: string[];
        /** 如果为 true，则替换现有显示的徽章。全局和页面范围的徽章分别独立覆盖。 **/
        override?: boolean;
        /** 如果为 true，则从租户配置更新徽章显示属性。 **/
        update?: boolean;
    };
}
[inline-code-end]

#### 版主和管理员

对于管理员和版主，在 `SSOUser` 对象中传递相应的 `isAdmin` 或 `isModerator` 标志。

#### 通知

要启用或禁用通知，请将 `optedInNotifications` 的值分别设置为 `true` 或 `false`。当用户第一次在 SSO 有效负载中加载页面时，其通知设置将被更新。

此外，如果您希望用户接收有关他们订阅页面活动的电子邮件通知（而不仅仅是应用内通知），请将 `optedInSubscriptionNotifications` 设置为 `true`。

#### VIP 用户 & 特殊标签

您可以使用可选的 "displayLabel" 字段在用户名称旁显示特殊标签。

#### 未认证用户

要表示未认证用户，只需不填充 userDataJSONBase64、verificationHash 或 timestamp。提供一个 loginURL 即可。

这些用户将无法发表评论，而是会看到一个登录提示（消息、链接或按钮，取决于配置）。

#### 序列化和哈希用户数据的直接示例

更多示例详见 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">此处（js）</a>、<a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">此处（java）</a> 和 <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">此处（php）</a>。

我们理解任何集成都可能是复杂且令人头疼的过程。如有疑问，请随时联系您的客户代表或使用 <a href="https://fastcomments.com/auth/my-account/help" target="_blank">支持页面</a>。

---