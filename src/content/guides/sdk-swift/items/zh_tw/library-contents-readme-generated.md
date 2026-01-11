FastComments Swift SDK 由數個模組組成：

- **Client Module** - 為 FastComments REST APIs 自動產生的 API 用戶端
  - 所有 API 模型的完整型別定義
  - 同時包含已授權的 (`DefaultAPI`) 與公開的 (`PublicAPI`) 端點
  - 完整的 async/await 支援
  - 詳細 API 文件請見 [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md)

- **SSO Module** - 伺服器端單一登入工具
  - 用於使用者驗證的安全 token 生成
  - 支援簡易與安全兩種 SSO 模式
  - 使用 CryptoKit 進行以 HMAC-SHA256 為基礎的 token 簽名