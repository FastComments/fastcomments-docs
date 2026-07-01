### 使用 Nimble

```bash
nimble install fastcomments
```

### 从源码构建

```bash
nimble build
```

### 库内容

此库包含生成的 API 客户端和 SSO 实用程序，以便更轻松地使用 API。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公开 API 与受保护 API

对于 API 客户端，有三个 API 模块，`api_default`、`api_public` 和 `api_moderation`。`api_default` 包含需要您的 API 密钥的方法，`api_public` 包含可以直接从浏览器/移动设备等发起且无需认证的 API 调用。`api_moderation` 模块包含用于审核员仪表板的方法。

`api_moderation` 模块提供了一套全面的实时且快速的审核 API。每个 `api_moderation` 方法都接受一个 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 cookie 进行身份验证。