[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

使用簡易 SSO，我們可以向評論小工具提供使用者資訊，讓他們在發表評論時不必輸入使用者名稱或電子郵件。

我們可以如下設定簡易 SSO：

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = '簡易 SSO'; code-example-end]

使用者將會被登入，並在背後建立一個 SSO 使用者。若從 API 取得，該使用者的 `createdFromSimpleSSO` 會被設定為 `true`。

注意事項：

- 電子郵件是簡易 SSO 的唯一識別碼。
- 提供電子郵件給簡易 SSO 並非必要，但預設情況下，他們的評論會顯示為「未驗證」。<b>如果未提供電子郵件，使用者將無法完成完整驗證。</b>
- **新增** 自 2022 年 1 月起：使用者名稱不必在 fastcomments.com 上全域唯一。
- 簡易 SSO 可以自動建立與更新 SSO 使用者（前提是提供了電子郵件，且該使用者最初不是由 Secure SSO 建立）。如果 Simple SSO 的 payload 指定了一個由 Secure SSO 或 API 建立的使用者，該使用者將保持不變，訪客則會獲得一個獨立的使用者。
- 您可以使用 `badgeConfig` 屬性為使用者指定徽章。`badgeIds` 陣列包含要與使用者關聯的全域徽章 ID。`pageBadgeIds` 陣列則包含針對當前頁面（`urlId`）的徽章 ID——這些徽章僅在被指派的頁面上顯示。若 `override` 設為 `true`，則會取代已顯示的徽章（全域與頁面範圍的徽章分別被覆寫）；若為 `false`，則會加入現有徽章。

#### 簡易 SSO 設定

簡易 SSO 永遠可用。兩個設定決定 FastComments 對其的信任程度。您可以在 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">自訂規則</a> 中搜尋 SSO 來找到它們。兩者預設皆為關閉，且不論「需要 SSO 才能檢視評論」是否開啟，都會套用。

在新增這些設定時已使用簡易 SSO 的帳號會同時開啟兩項設定，因此對它們不會產生變化。

僅會套用最符合頁面的自訂規則，因此若您有多個規則，請在每個規則上設定這兩項。若您其他規則中有開啟，新的規則會預設為開啟。

##### 透過簡易 SSO 自動驗證評論

開啟此設定時，來自簡易 SSO 使用者的評論會立即驗證。

關閉此設定時，簡易 SSO 使用者的評論會顯示為「未驗證」，直到使用者點擊我們寄送至 Simple SSO payload 中電子郵件地址的連結。若您僅自動批准已驗證的評論，這些評論也會等待該連結。若 payload 中沒有電子郵件，評論將保持未驗證。

此設定僅影響評論。來自簡易 SSO 使用者的投票始終會立即計算。

##### 允許簡易 SSO 管理員

開啟此設定時，簡易 SSO 使用者可以成為您帳號的管理員或版主：

- 擁有管理員或版主權限的簡易 SSO 使用者會保留這些權限。
- 若簡易 SSO 使用者的電子郵件與您其中一位管理員或版主相同，則會取得其權限。

關閉此設定時，簡易 SSO 不會授予這些權限。若 Simple SSO payload 指定了一位擁有此類權限的使用者，則會產生一個沒有權限的獨立使用者，原始使用者保持不變。

Simple SSO payload 未簽名，因此開啟此設定時，任何知道管理員電子郵件或使用者 ID 的人都可以冒充其身份。若這對您的網站很重要，請關閉此設定，並使用 Secure SSO 為您的管理員提供安全驗證。

由 Secure SSO 或 API 建立的使用者無法被 Simple SSO 取得，無論此設定是否開啟。