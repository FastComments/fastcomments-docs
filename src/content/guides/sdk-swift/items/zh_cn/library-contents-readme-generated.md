FastComments Swift SDK 由多个模块组成：

- **客户端模块** - FastComments REST API 的自动生成客户端
  - 为所有 API 模型提供完整的类型定义
  - 同时包含已认证（`DefaultAPI`）和公开（`PublicAPI`）端点
  - 完整的 async/await 支持
  - 参见 [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) 以获取详细的 API 文档

- **SSO 模块** - 服务器端单点登录工具
  - 用于用户身份验证的安全令牌生成
  - 支持简单和安全两种 SSO 模式
  - 使用 CryptoKit 进行基于 HMAC-SHA256 的令牌签名