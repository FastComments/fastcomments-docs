---
FastComments 透過 SSO（單一登入）與 Drupal 的使用者系統整合。使用者登入您的 Drupal 網站後，模組會自動將他們的身分傳送到 FastComments。無需建立額外帳號，也不需要執行初始同步。

The module supports three SSO modes, set under `Administration > Configuration > Content > FastComments`.

### 無

No SSO。使用者可以訪客身份留言或建立 FastComments 帳號。若您的網站公開且不需要將留言綁定到 Drupal 使用者，請使用此模式。

### 簡易

在沒有伺服器端驗證的情況下，將 Drupal 使用者的名稱、電子郵件與大頭照傳給 FastComments。無需 API Secret。適用於內部或低風險網站。

### 安全（建議）

使用 [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) 來向 FastComments 驗證每個使用者的身分。若您已設定 API Secret，此模式即為您所需，且它是唯一能防止訪客冒充其他使用者的模式。

User identity is passed to FastComments each time a user views a comment thread. There is no initial or continuous sync that needs to run.

<sup>(Optional)</sup> 將您的管理員加入 [Users & Administrators](https://fastcomments.com/auth/my-account/users)，並將版主加入 [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators)，以改善他們的使用體驗並啟用版主的統計追蹤。

如要深入了解 SSO 的運作方式，請參閱自訂文件的 [SSO 章節](/guide-customizations-and-configuration.html#sso)。

---