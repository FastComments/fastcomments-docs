該外掛支援三種 SSO 模式，用以將 Moodle 的使用者帳號與 FastComments 整合。

#### Secure SSO (Recommended)

使用 HMAC-SHA256 與您的 API Secret 在伺服器端對使用者資料進行簽章。這是最安全的選項，建議用於生產環境。

使用 Secure SSO：

- 使用者名稱、電子郵件與頭像會安全地傳送到 FastComments。
- Moodle 站台管理員會自動同步為 FastComments 管理員。
- 使用者無法冒充其他帳號。
- 需要在外掛設定中設定 **API Secret**。

#### Simple SSO

使用者資料（姓名、電子郵件、頭像）在用戶端傳送，未包含加密簽章。這比較容易設定，因為不需要 API Secret，但較不安全，因為使用者資料未經伺服器端驗證。

#### None

無 SSO 整合。使用者以匿名方式留言或必須另外登入 FastComments。若您不希望 Moodle 使用者帳號連結到留言，請使用此模式。