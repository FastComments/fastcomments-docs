---
The FastComments Rust SDK 由若干模块组成：

- **Client Module** - 为 FastComments REST API 自动生成的 API 客户端
  - 提供所有 API 模型的完整类型定义
  - 包含已认证的（`DefaultApi`）和公共的（`PublicApi`）端点
  - 使用 tokio 完全支持 async/await
  - 详见 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) 获取详细的 API 文档

- **SSO Module** - 服务器端单点登录实用工具
  - 用于用户认证的安全令牌生成
  - 支持简单和安全两种 SSO 模式
  - 基于 HMAC-SHA256 的令牌签名

- **Core Types** - 共享类型定义和实用工具
  - 评论模型和元数据结构
  - 用户和租户配置
  - 常用操作的辅助函数
---