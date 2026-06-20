---
FastComments 的 Rust SDK 由多个模块组成：

- **Client Module** - FastComments REST API 的 API 客户端
  - 为所有 API 模型提供完整的类型定义
  - 覆盖所有 FastComments 方法的三种 API 客户端：
    - `default_api` (**DefaultApi**) - 供服务器端使用的基于 API 密钥认证的方法
    - `public_api` (**PublicApi**) - 公开的、无需 API 密钥的方法，适合在浏览器和移动应用中调用
    - `moderation_api` (**ModerationApi**) - 支持版主仪表盘的方法，包括评论审核（列表、计数、搜索、日志、导出）、审核操作（移除/恢复、标记、设置审核/垃圾/审批状态、投票、重新打开/关闭线程）、封禁（基于评论的封禁、撤销、封禁前摘要、封禁状态/偏好、被封禁用户计数）以及徽章与信任（授予/移除徽章、手动徽章、获取/设置信任因子、用户内部资料）。每个 Moderation 方法都接受一个 `sso` 参数，因此该调用可以以 SSO 验证的版主的身份进行。
  - 使用 tokio 完整支持 async/await
  - 详见 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) 获取详细的 API 文档

- **SSO Module** - 服务器端单点登录工具
  - 用于用户认证的安全令牌生成
  - 支持简单与安全两种 SSO 模式
  - 基于 HMAC-SHA256 的令牌签名

- **Core Types** - 共享的类型定义和实用工具
  - 评论模型与元数据结构
  - 用户与租户配置
  - 常用操作的辅助函数
---