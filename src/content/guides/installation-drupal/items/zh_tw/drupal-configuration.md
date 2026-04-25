All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## 必填

- **租戶 ID** - 您的 FastComments Tenant ID。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）中找到。
- **API Secret** - 用於安全 SSO、Webhook 驗證與頁面同步。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）中找到。

## 留言樣式

選擇與您網站互動方式相符的小工具。

- **Live Comments** - 即時的分線留言。
- **Streaming Chat** - 即時聊天介面，適用於活動與直播。
- **Collab Chat** - 在主要內容區進行文字選取註解。訪客選取文字並在該處開始討論。
- **Collab Chat + Comments** - 在同一頁面同時顯示 collab chat 與標準留言。

## SSO 模式

- **無** - 不使用 SSO。使用者以訪客身份留言或建立 FastComments 帳號。
- **簡易** - 將 Drupal 使用者資訊（名稱、電子郵件、頭像）傳給 FastComments，無需伺服器端驗證。
- **安全** - 使用 HMAC-SHA256 與 FastComments 驗證 Drupal 使用者。當您已設定 API Secret 時建議使用。

See the `Single Sign-On (SSO)` section for details.

## 其他設定

- **CDN URL** - 預設為 `https://cdn.fastcomments.com`。
- **Site URL** - 預設為 `https://fastcomments.com`。
- **Email notifications** - 當新的留言發表於其內容時，寄送電子郵件給內容作者。

For EU data residency, see the `EU Data Residency` section.