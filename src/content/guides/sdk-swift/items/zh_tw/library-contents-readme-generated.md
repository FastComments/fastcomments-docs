FastComments Swift SDK 包含數個模組：

- **Client Module** - 用於 FastComments REST APIs 的 API 用戶端
  - 提供所有 API 模型的完整型別定義
  - 已驗證 (`DefaultAPI`)、公開 (`PublicAPI`)、以及審核 (`ModerationAPI`) 方法
  - 完整的 async/await 支援
  - 詳細的 API 文件請參見 [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md)

- **SSO Module** - 伺服器端的 Single Sign-On 實用工具
  - 用於使用者驗證的安全 token 產生
  - 支援簡易與安全兩種 SSO 模式
  - 使用 CryptoKit 的基於 HMAC-SHA256 的 token 簽名