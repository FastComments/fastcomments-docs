### 从 GitHub 安装

直接从发布标签安装（推荐，完全可复现）：

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

固定标签而不是分支，以确保构建确定性。相同的写法也适用于 `requirements.txt`：

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

每个带标签的 [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) 也附带了已构建的 wheel，如果你更倾向于直接安装二进制制品，可以使用它。

### 库内容

该库包含两个模块：生成的 API 客户端和核心 Python 库，后者包含手写的实用工具，以简化 API 的使用，包括 SSO 支持。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心库文档，含 SSO 示例](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公共 API 与受限 API

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要你的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等无需身份验证调用的方法。`ModerationApi` 提供了丰富的实时和快速审核 API。每个 `ModerationApi` 方法都接受 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 Cookie 进行身份验证。