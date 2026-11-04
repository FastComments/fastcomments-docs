将此行添加到您应用程序的 Gemfile 中：

```ruby
gem 'fastcomments'
```

然后执行：

```bash
bundle install
```

或者自行安装为：

```bash
gem install fastcomments
```

### 库内容

该库包含生成的 API 客户端以及使使用 API 更方便的 SSO 工具。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公开 API 与受保护 API

对于 API 客户端，有三个类：`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，而 `PublicApi` 包含可以直接从浏览器/移动设备等无需身份验证即可调用的 API 调用。`ModerationApi` 包含为审核员仪表板提供支持的方法。

`ModerationApi` 提供了一套广泛、实时且快速的审核 API。每个 `ModerationApi` 方法都接受一个 `sso` 参数，并可以通过 SSO 或 FastComments.com 会话 cookie 进行身份验证。