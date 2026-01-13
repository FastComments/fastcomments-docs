将此行添加到您应用程序的 Gemfile：

```ruby
gem 'fastcomments'
```

然后执行：

```bash
bundle install
```

或者您也可以这样安装：

```bash
gem install fastcomments
```

### 库内容

该库包含生成的 API 客户端和 SSO 实用工具，以便更轻松地使用 API。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公共 API 与受保护 API

对于 API 客户端，有两个类：`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要您的 API 密钥的方法，而 `PublicApi` 包含可直接从浏览器/移动设备等发起且无需身份验证的 API 调用。
---