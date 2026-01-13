[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

使用 Simple SSO，我们可以向评论小部件提供用户信息，这样他们在发表评论时无需输入用户名或电子邮件。

我们可以按如下方式配置 Simple SSO：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

用户将会登录，并在后台创建一个 SSO 用户。如果从 API 获取，该用户的 `createdFromSimpleSSO` 将被设置为 `true`。

Notes: 

- 电子邮件是 Simple SSO 的唯一标识符。
- 向 Simple SSO 提供电子邮件不是必需的，但默认情况下，他们的评论将显示为 “未验证”。 <b>如果未提供电子邮件，则用户无法完全通过身份验证。</b>
- **新** 自 2022 年 1 月起：用户名在整个 fastcomments.com 上不必唯一
- 如果提供了电子邮件，且该用户最初不是通过 Secure SSO 创建的，Simple SSO 可以自动创建和更新 SSO 用户。
- 您可以使用 `badgeConfig` 属性为用户指定徽章。`badgeIds` 数组包含要与用户关联的徽章 ID。如果 `override` 设置为 `true`，它将替换评论中显示的所有现有徽章；如果为 `false`，则会在现有徽章上追加。

---