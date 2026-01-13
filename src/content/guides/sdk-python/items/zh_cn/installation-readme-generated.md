---
### PyPI

```bash
pip install fastcomments
```

### 库内容

该库包含两个模块：生成的 API 客户端和核心 Python 库，后者包含手写的实用工具以简化与 API 的交互，包括 SSO 支持。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心库文档（包括 SSO 示例）](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公开与受保护的 API

对于 API 客户端，有两个类，`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等在无身份验证情况下调用的 API。
---