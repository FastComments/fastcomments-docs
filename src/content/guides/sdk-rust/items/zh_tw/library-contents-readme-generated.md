---
FastComments Rust SDK 由數個模組組成：

- **Client Module** - 自動生成的 FastComments REST API 用戶端
  - 針對所有 API 模型的完整型別定義
  - 同時包含已驗證的 (`DefaultApi`) 與公開的 (`PublicApi`) 端點
  - 使用 tokio 提供完整的 async/await 支援
  - 詳細 API 文件請見 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)

- **SSO Module** - 伺服器端單一登入工具
  - 為使用者驗證產生安全的權杖
  - 支援簡單與安全兩種 SSO 模式
  - 基於 HMAC-SHA256 的權杖簽名

- **Core Types** - 共用型別定義與工具
  - 評論模型與中繼資料結構
  - 使用者與租戶設定
  - 常用操作的輔助函式
---