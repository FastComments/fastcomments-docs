外掛設定頁面位於 **網站管理 > 外掛 > 本機外掛 > FastComments**。可用選項如下：

#### Tenant ID

您的 FastComments 租戶 ID。可在 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 儀表板</a> 的帳戶設定中找到。

#### API Secret

您的 API 密鑰，使用安全 SSO 模式時必需。可在 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">我的帳戶 > API Secret</a> 找到此項。

#### SSO Mode

選擇使用者的驗證方式。請參閱 [SSO Modes](#moodle-sso-modes) 區段以了解各選項的詳細說明。

- **Secure** (recommended) - 伺服器端 HMAC-SHA256 簽章驗證
- **Simple** - 客戶端使用者資料，無簽章
- **None** - 匿名留言，未整合 Moodle 登入

#### Page Contexts

控制留言出現的位置：

- **Course pages** - 在課程主頁的留言
- **Module/activity pages** - 在個別活動與資源頁面的留言
- **Both** - 在所有頁面類型顯示留言

#### Commenting Style

選擇留言體驗。請參閱 [Commenting Styles](#moodle-commenting-styles) 以查看每種模式的畫面截圖。

- **Comments** - 頁面內容下方的標準串狀留言元件
- **Collab Chat** - 內嵌文字選取討論，並顯示在線狀態指示
- **Both** - 同時啟用留言與協作聊天

#### CDN URL

FastComments 的 CDN URL。預設為 `https://cdn.fastcomments.com`。若您的資料託管於 EU 區域，請將此改為 EU 的 CDN URL。