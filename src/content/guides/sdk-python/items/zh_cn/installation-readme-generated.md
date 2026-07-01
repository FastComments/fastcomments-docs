### 从 GitHub 安装

直接从发布标签安装（推荐，完全可复现）：

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

固定标签而不是分支，以确保构建确定性。同样的写法可以在 `requirements.txt` 中使用：

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

每个带标签的 [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) 也附带了构建好的 wheel，如果您更倾向直接安装二进制工件的话。

### 库内容

此库包含两个模块：生成的 API 客户端以及包含手写工具的核心 Python 库，这些工具能够让使用 API 更加方便，包括 SSO 支持。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公开 API 与受限 API

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等无认证情况下调用的方法。`ModerationApi` 提供了丰富的实时快速审核 API 套件。每个 `ModerationApi` 方法都接受 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 Cookie 进行身份验证。