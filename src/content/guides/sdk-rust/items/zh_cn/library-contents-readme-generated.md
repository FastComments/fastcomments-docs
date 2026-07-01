The FastComments Rust SDK 包含若干模块：

- **Client Module** - FastComments REST API 的 API 客户端
  - 完整的所有 API 模型的类型定义
  - 三个 API 客户端，覆盖所有 FastComments 方法：
    - `default_api` (**DefaultApi**) - 通过 API 密钥进行身份验证的服务器端使用方法
    - `public_api` (**PublicApi**) - 公共的、无需 API 密钥的方法，可安全地从浏览器和移动应用调用
    - `moderation_api` (**ModerationApi**) - 一个广泛的实时快速审核 API 套件。每个 Moderation 方法接受 `sso` 参数，并可通过 SSO 或 FastComments.com 会话 cookie 进行身份验证。
  - 完整的基于 tokio 的 async/await 支持
  - 详尽的 API 文档请参阅 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)

- **SSO Module** - 服务器端单点登录工具
  - 为用户认证生成安全令牌
  - 支持简易和安全两种 SSO 模式
  - 基于 HMAC-SHA256 的令牌签名

- **Core Types** - 共享的类型定义和工具
  - 评论模型和元数据结构
  - 用户和租户配置
  - 常用操作的辅助函数