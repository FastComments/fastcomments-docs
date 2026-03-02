外掛設定頁面位於 **網站管理 > 外掛 > 本地外掛 > FastComments**。可用選項如下：

#### Tenant ID

您的 FastComments 租戶 ID。可在 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 儀表板</a> 的帳戶設定中找到。

#### API Secret

您的 API Secret 金鑰，Secure SSO 模式所需。可在 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">我的帳戶 > API Secret</a> 找到。

#### SSO 模式

選擇使用者的驗證方式。請參閱 [SSO 模式](#items-moodle-sso-modes) 欄目以瞭解各選項的詳細說明。

- **Secure**（建議） - 伺服器端 HMAC-SHA256 簽名驗證
- **Simple** - 用戶端的使用者資料，無簽名
- **None** - 匿名留言，未整合 Moodle 登入

#### 頁面上下文

控制留言顯示的位置：

- **Course pages** - 在課程主頁面顯示留言
- **Module/activity pages** - 在各活動與資源的個別頁面顯示留言
- **Both** - 在所有頁面類型顯示留言

#### 留言樣式

選擇留言體驗。請參閱 [留言樣式](#items-moodle-commenting-styles) 查看各模式的截圖。

- **Comments** - 標準的分層留言元件，顯示於頁面內容下方
- **Collab Chat** - 內嵌文字選取討論，顯示在線狀態
- **Both** - 同時啟用留言與協作聊天

#### CDN URL

FastComments 的 CDN URL。預設為 `https://cdn.fastcomments.com`。如果您的資料託管在歐盟區域，請將此變更為 EU 的 CDN URL。

---