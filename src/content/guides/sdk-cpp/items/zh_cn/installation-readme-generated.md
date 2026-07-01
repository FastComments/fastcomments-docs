### 安装依赖

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### 从源码构建

```bash
mkdir build
cd build
cmake ..
make
```

### 安装

```bash
sudo make install
```

### 库内容

此库包含生成的 API 客户端和 SSO 实用工具，以简化 API 的使用。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公共 API 与受保护 API

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器、移动设备等发起且无需身份验证的方法。`ModerationApi` 提供了一套完整且快速的实时审核 API。每个 `ModerationApi` 方法都接受 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 Cookie 进行身份验证。