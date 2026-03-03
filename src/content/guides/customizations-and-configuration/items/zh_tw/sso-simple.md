[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

使用 Simple SSO，我們可以向評論小工具提供使用者資訊，讓他們在留言時不必輸入使用者名稱或電子郵件。

我們可以如下配置 Simple SSO：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

使用者將會被登入，並在後台建立一個 SSO 使用者。如果從 API 擷取，該使用者的 `createdFromSimpleSSO` 將會被設為 `true`。

Notes: 

- 電子郵件是 Simple SSO 的唯一識別符。
- 在 Simple SSO 中提供電子郵件不是必要的，但預設情況下，他們的評論會顯示為「Unverified」。 <b>如果未提供電子郵件，該使用者無法被完全驗證。</b>
- **NEW** 自 2022 年 1 月起：使用者名稱不必在整個 fastcomments.com 上唯一
- 如果提供電子郵件，且該使用者最初不是由 Secure SSO 建立，Simple SSO 可以自動建立和更新 SSO 使用者。
- 你可以使用 `badgeConfig` 屬性為使用者指定徽章。`badgeIds` 陣列包含要與使用者關聯的全域徽章 ID。`pageBadgeIds` 陣列包含限定於當前頁面（`urlId`）的徽章 ID — 這些徽章只會在被指派的該頁面上顯示。如果 `override` 設為 `true`，將會取代現有顯示的徽章（全域與頁面範圍徽章分別獨立覆寫）；若為 `false` 則會加入到現有徽章中。