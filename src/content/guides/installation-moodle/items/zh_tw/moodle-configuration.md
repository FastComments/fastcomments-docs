外掛設定頁面位於 **網站管理 > 外掛 > 本地外掛 > FastComments**。可用的選項有：

#### 租戶 ID (Tenant ID)

您的 FastComments 租戶 ID。請在 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 儀表板</a> 的帳戶設定中找到此資訊。

#### API 密鑰 (API Secret)

您的 API Secret 金鑰，安全 SSO 模式需要此金鑰。請在 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">我的帳戶 > API Secret</a> 找到此資訊。

#### SSO 模式

選擇使用者驗證的方式。各選項的詳細說明請參閱 [SSO 模式](#moodle-sso-modes) 章節。

- **安全 (Secure)**（建議）- 伺服器端 HMAC-SHA256 簽署驗證
- **簡單 (Simple)** - 用戶端使用者資料，無簽署
- **無 (None)** - 匿名留言，不與 Moodle 登入整合

#### 頁面情境 (Page Contexts)

控制留言顯示的位置：

- **課程頁面** - 課程主頁面上的留言
- **模組/活動頁面** - 個別活動與資源上的留言
- **兩者** - 所有頁面類型上的留言

#### 留言風格 (Commenting Style)

選擇留言體驗。各模式的截圖請參閱 [留言風格](#moodle-commenting-styles)。

- **留言 (Comments)** - 頁面內容下方的標準巢狀留言小工具
- **協作聊天 (Collab Chat)** - 內文選取討論，並具備在線狀態指示
- **兩者** - 留言與協作聊天同時啟用

#### CDN URL

FastComments 的 CDN URL。預設為 `https://cdn.fastcomments.com`。若您的資料託管於歐盟 (EU) 地區，請將此更改為歐盟 CDN URL。