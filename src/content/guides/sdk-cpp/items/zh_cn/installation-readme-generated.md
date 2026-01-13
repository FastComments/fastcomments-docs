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

该库包含生成的 API 客户端和 SSO 实用工具，使与 API 的集成更简单。

- [API 客户端库 文档](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公共与受保护的 API

对于 API 客户端，有两个类：`DefaultAPI` 和 `PublicAPI`。`DefaultAPI` 包含需要使用您 API 密钥的方法，`PublicAPI` 包含 API 调用
可以直接从浏览器/移动设备等发起且无需认证。