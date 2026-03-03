[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

使用 Simple SSO，我们可以向评论小部件提供有关用户的信息，这样他们就不必在发表评论时输入用户名或电子邮件地址。

我们可以按如下方式配置 Simple SSO：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

用户将被登录，并将在后台创建一个 SSO 用户。如果从 API 获取该用户，则该用户的 `createdFromSimpleSSO` 将被设置为 `true`。

Notes: 

- 电子邮件是 Simple SSO 的唯一标识符。
- 使用 Simple SSO 提供电子邮件并不是必需的，但默认情况下他们的评论将显示为 "Unverified"。 <b>如果未提供电子邮件，用户将无法完成完全身份验证。</b>
- **NEW** 自 2022 年 1 月起：用户名不必在整个 fastcomments.com 上唯一
- 如果提供了电子邮件，且该用户最初不是通过 Secure SSO 创建的，Simple SSO 可以自动创建和更新 SSO 用户。
- 你可以通过 `badgeConfig` 属性为用户指定徽章。`badgeIds` 数组包含要与用户关联的全局徽章 ID。`pageBadgeIds` 数组包含作用于当前页面（`urlId`）的徽章 ID — 这些徽章仅在分配它们的页面上显示。如果将 `override` 设置为 `true`，它将替换现有显示的徽章（全局和页面范围的徽章分别独立覆盖）；如果为 `false`，则会在现有徽章上追加。