### 使用 Nimble

```bash
nimble install fastcomments
```

### 从源码构建

```bash
nimble build
```

### 库内容

此库包含生成的 API 客户端和 SSO 实用程序，以便更方便地使用 API。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公共与受保护的 API

对于 API 客户端，有两个 API 模块，`api_default` 和 `api_public`。`api_default` 包含需要您的 API 密钥的方法，`api_public` 包含可以直接从浏览器/移动设备/等在无需身份验证的情况下调用的 API。