### 使用 Nimble

```bash
nimble install fastcomments
```

### 从源码构建

```bash
nimble build
```

### 库内容

此库包含生成的 API 客户端和 SSO 实用工具，以便更容易地使用该 API。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公开与受保护的 API

对于 API 客户端，有三个 API 模块：`api_default`、`api_public` 和 `api_moderation`。`api_default` 包含需要您的 API 密钥的方法，`api_public` 包含可直接从浏览器/移动设备等发出的无需认证的 API 调用。`api_moderation` 模块包含供版主面板使用的方法。

`api_moderation` 方法涵盖列出、计数、搜索和导出评论及其日志；管理操作，如删除/恢复评论、标记、设置审核/垃圾/批准状态、调整投票以及重新打开/关闭主题；封禁（针对评论封禁用户、撤销封禁、封禁前摘要、封禁状态和偏好，以及被封禁用户计数）；以及徽章与信任（授予/移除徽章、列出手动徽章、获取/设置用户的信任系数，以及获取用户的内部档案）。每个 `api_moderation` 方法都接受一个 `sso` 参数，以便该调用以 SSO 版主身份进行身份验证。