### PyPI

```bash
pip install fastcomments
```

### 库内容

此库包含两个模块：生成的 API 客户端和核心 Python 库，后者包含手写的实用工具以简化对 API 的使用，包括 SSO 支持。

- [API 客户端库 文档](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心库文档，包含 SSO 示例](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公开与受保护的 API

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等在无需认证的情况下调用的方法。`ModerationApi` 为版主控制面板提供支持，包含用于审核评论的方法（列表、计数、搜索、日志、导出）、审核操作（移除/恢复、标记、设置审核/垃圾/批准状态、投票、重新打开/关闭 线程）、封禁（禁止评论、撤销、预封禁摘要、封禁状态/偏好、被封禁用户计数）以及徽章与信任（授予/移除徽章、手动徽章、获取/设置信任因子、用户内部资料）。每个 `ModerationApi` 方法都接受一个 `sso` 参数，因此可以以已通过 SSO 认证的版主身份进行调用。