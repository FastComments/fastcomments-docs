SDK 透過 `CommentWidgetConfig` 類別提供許多配置選項：

| 選項 | 說明 |
|--------|-------------|
| `tenantId` | 您的 FastComments 帳號 ID |
| `urlId` | 代表目前頁面的 ID |
| `sso` | 用於驗證的 SSO 權杖 |
| `allowAnon` | 允許匿名留言 |
| `voteStyle` | UpDown 或 Heart 投票樣式 |
| `hideAvatars` | 隱藏使用者頭像 |
| `hasDarkBackground` | 表示深色模式 |
| `customCSS` | 自訂 CSS 樣式 |
| `enableInfiniteScrolling` | 啟用無限捲動分頁 |
| `readonly` | 停用留言但仍顯示留言 |
| `disableVoting` | 停用投票功能 |
| `disableLiveCommenting` | 停用即時更新 |
| `pollCreation` | 誰可以在留言上附加投票：`NUMBER_0` 停用、`NUMBER_1` 管理員與版主、`NUMBER_2` 所有人 |
| `pollVoting` | 誰可以對投票進行投票：`NUMBER_0` 已登入使用者、`NUMBER_1` 任何人 |