Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

此库包含生成的 API 客户端以及用于简化 API 使用的 SSO 实用工具。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等无需身份验证即可调用的 API。`ModerationApi` 包含为管理员仪表板提供动力的方法。

`ModerationApi` 提供了功能丰富且快速的实时审核 API 套件。每个 `ModerationApi` 方法都接受一个 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 cookie 进行身份验证。