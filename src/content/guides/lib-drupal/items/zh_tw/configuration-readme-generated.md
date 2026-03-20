前往 **管理 > 設定 > 內容 > FastComments** (`/admin/config/content/fastcomments`)。

### 設定

- **租戶 ID**（必填） - 您的 FastComments 租戶 ID。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）找到。
- **API 密鑰** - 用於安全 SSO、Webhooks 驗證與頁面同步。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）找到。
- **SSO 模式** - 單一登入整合：
  - **None** - 無 SSO，使用者以訪客身分留言或建立 FastComments 帳號。
  - **Simple** - 將 Drupal 使用者資訊（姓名、電子郵件、大頭貼）傳給 FastComments，無伺服器端驗證。
  - **Secure** - 使用 HMAC-SHA256 驗證以安全地讓 Drupal 使用者與 FastComments 驗證（建議）。
- **評論樣式** - 要顯示的元件類型：
  - **Live Comments** - 即時串列回覆式評論。
  - **Streaming Chat** - 即時聊天介面。
  - **Collab Chat** - 在主要內容區進行協作文字選取註解。
  - **Collab Chat + Comments** - 同時顯示協作聊天與標準評論。
- **CDN URL** - FastComments CDN URL（預設：`https://cdn.fastcomments.com`）。
- **Site URL** - FastComments 網站 URL（預設：`https://fastcomments.com`）。
- **電子郵件通知** - 當內容有新評論時，寄送電子郵件給內容作者。

### 將評論新增至內容類型

透過 **Structure > Content types > [type] > Manage fields** 將 **FastComments** 欄位新增到您的內容類型。此欄位具有啟用/停用切換，以及可為每個實體設定的選用自訂識別碼。

### 歐盟資料駐留

若需歐盟資料駐留，請更新：
- **CDN URL** 為 `https://cdn-eu.fastcomments.com`
- **Site URL** 為 `https://eu.fastcomments.com`