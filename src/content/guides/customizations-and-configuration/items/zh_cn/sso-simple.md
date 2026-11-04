[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

使用 Simple SSO，我们可以向评论小部件提供用户信息，这样用户就不必输入用户名或电子邮件即可发表评论。

我们可以按如下方式配置 Simple SSO：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

用户将被登录，并在后台创建一个 SSO 用户。如果从 API 获取，用户的 `createdFromSimpleSSO` 将被设置为 `true`。

注意事项：

- 电子邮件是 Simple SSO 的唯一标识符。
- 提供电子邮件并非必需，但默认情况下他们的评论会显示为 “未验证”。<b>如果未提供电子邮件，用户将无法完成完整的身份验证。</b>
- **新** 自 2022 年 1 月起：用户名不必在整个 fastcomments.com 上唯一。
- Simple SSO 可以在提供电子邮件且用户最初不是由 Secure SSO 创建的情况下自动创建和更新 SSO 用户。如果 Simple SSO 负载中指定的用户是由 Secure SSO 或 API 创建的，则该用户保持不变，访客将获得一个单独的用户。
- 您可以使用 `badgeConfig` 属性为用户指定徽章。`badgeIds` 数组包含要关联到用户的全局徽章 ID。`pageBadgeIds` 数组包含针对当前页面 (`urlId`) 的徽章 ID——这些徽章仅在分配它们的页面上显示。如果 `override` 设置为 `true`，它将替换已有的显示徽章（全局和页面范围的徽章分别被覆盖）；如果为 `false`，则会在已有徽章上追加。

#### Simple SSO 设置

Simple SSO 始终可用。两个设置控制 FastComments 对其的信任程度。您可以在 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自定义规则</a> 中搜索 SSO 找到它们。默认情况下这两个设置均关闭，并且它们在 “需要 SSO 才能查看评论” 是否开启的情况下都适用。

在添加这些设置时已经使用 Simple SSO 的账户已将两者都打开，因此对它们没有影响。

只有最匹配页面的自定义规则会生效，所以如果您有多个规则，需要在每个规则上都设置这些选项。若您其他规则已打开这些选项，新规则默认也会打开。

##### 通过 Simple SSO 自动验证评论

开启此选项后，来自 Simple SSO 用户的评论会立即被验证。

关闭此选项后，来自 Simple SSO 用户的评论会显示为 “未验证”，直到用户点击我们发送到 Simple SSO 负载中电子邮件地址的链接。如果您只自动批准已验证的评论，它们也会等待该链接。如果负载中没有电子邮件，评论将保持未验证。

此设置仅影响评论。来自 Simple SSO 用户的投票始终会立即计入。

##### 允许 Simple SSO 管理员

开启此选项后，Simple SSO 用户可以成为您账户的管理员或版主：

- 拥有管理员或版主权限的 Simple SSO 用户会保留这些权限。
- 与您某位管理员或版主的电子邮件相同的 Simple SSO 用户会获得相同的权限。

关闭此选项后，Simple SSO 不会授予这些权限。若 Simple SSO 负载中指定的用户拥有这些权限，则会创建一个没有权限的单独用户，原始用户保持不变。

Simple SSO 负载未签名，因此开启此选项后，任何知道管理员电子邮件或用户 ID 的人都可以冒充他们。如果这对您的站点很重要，请关闭此选项并使用 Secure SSO 为管理员提供身份验证。

由 Secure SSO 或 API 创建的用户无法被 Simple SSO 访问，无论此选项是否开启。