所有設定位於 `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) 下。

## 必要設定

- **Tenant ID** - 您的 FastComments 租戶 ID。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）中找到。
- **API Secret** - 用於安全 SSO、Webhook 驗證與頁面同步的必備資訊。可在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）中找到。

## 儲存 API Secret

模組會從以下來源中依序讀取第一個有值的 API Secret：

1. **Key module** - 安裝 [Key](https://www.drupal.org/project/key)，建立一個 *Authentication* 金鑰，並在設定表單的 **API Secret key** 中選取。此金鑰可由環境變數或網站根目錄外的檔案提供。
2. **settings.php** - 在 `settings.php` 或 `settings.local.php` 中加入 `$config['fastcomments.settings']['api_secret'] = 'your-secret';`。此密鑰不會寫入匯出的設定檔。
3. **設定表單** - 在 **API Secret** 欄位中輸入。此值會儲存在 `fastcomments.settings` 中，因而會出現在設定匯出。Drupal 狀態報告會在此為使用中的來源時發出警告。

表單會顯示目前使用的來源。將欄位留空可保留已儲存的密鑰。勾選 **Remove the API Secret stored in configuration** 可在將密鑰移至 Key 模組或 `settings.php` 後刪除它。

## 評論樣式

選擇符合您網站上使用者交流方式的元件。

- **Live Comments** - 即時的分支式評論。
- **Streaming Chat** - 直播聊天介面，適合活動與直播。
- **Collab Chat** - 於主要內容區域的文字選取註解。訪客可標記文字並在上下文中發起討論。
- **Collab Chat + Comments** - 同時在同一頁面提供協作聊天與標準評論。

## SSO 模式

- **None** - 不使用 SSO。使用者以訪客身分評論或建立 FastComments 帳號。
- **Simple** - 將 Drupal 使用者資訊（名稱、電子郵件、頭像）傳遞給 FastComments，無需伺服器端驗證。
- **Secure** - 使用 HMAC‑SHA256 來驗證 Drupal 使用者與 FastComments 的關聯。建議在已設定 API Secret 時使用。

請參閱 `Single Sign-On (SSO)` 章節以取得詳細資訊。

## 其他設定

- **CDN URL** - 預設為 `https://cdn.fastcomments.com`。
- **Site URL** - 預設為 `https://fastcomments.com`。
- **Email notifications** - 當有新評論發佈於內容上時，會向內容作者發送電子郵件通知。

關於 EU 資料居留，請參閱 `EU Data Residency` 章節。