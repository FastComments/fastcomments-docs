FastComments Rust SDK 由多個模組組成：

- **客戶端模組** - FastComments REST API 的 API 客戶端
  - 完整的所有 API 模型類型定義
  - 三個 API 客戶端，涵蓋所有 FastComments 方法：
    - `default_api` (**DefaultApi**) - 透過 API 金鑰驗證的伺服器端使用方法
    - `public_api` (**PublicApi**) - 公開的、無需 API 金鑰的方法，可安全在瀏覽器和行動應用中呼叫
    - `moderation_api` (**ModerationApi**) - 廣泛的即時且快速的審核 API 組合。每個 Moderation 方法接受 `sso` 參數，且可透過 SSO 或 FastComments.com 會話 Cookie 進行驗證。
  - 完整的 async/await 支援，使用 tokio
  - 請參閱 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) 以取得詳細的 API 文件說明

- **SSO 模組** - 伺服器端單一登入（Single Sign-On）工具
  - 安全的令牌產生，用於使用者驗證
  - 支援簡易與安全兩種 SSO 模式
  - 基於 HMAC‑SHA256 的令牌簽署

- **核心類型** - 共享的類型定義與工具函式
  - 評論模型與中繼資料結構
  - 使用者與租戶設定
  - 常用操作的輔助函式