将此行添加到您应用程序的 Gemfile：

```ruby
gem 'fastcomments'
```

然后执行：

```bash
bundle install
```

或者直接安装：

```bash
gem install fastcomments
```

### Library Contents

此库包含生成的 API 客户端和 SSO 实用工具，以便更轻松地使用 API。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等在不进行身份验证的情况下调用的 API 请求。`ModerationApi` 包含为审核员仪表板提供功能的方法。

`ModerationApi` 涵盖评论审核（列出、计数、搜索、日志、导出）、审核操作（移除/恢复、标记、设置审核/垃圾/审批状态、投票、重新打开/关闭线程）、封禁（从评论中封禁、撤销、预封禁摘要、封禁状态/偏好、被封用户计数）以及徽章与信任（授予/移除徽章、手动徽章、获取/设置信任因子、用户内部资料）。每个 `ModerationApi` 方法都接受一个 `sso` 参数，以便代表经过 SSO 验证的审核员发出请求。