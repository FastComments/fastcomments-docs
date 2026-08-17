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

此程式庫包含產生的 API 用戶端以及 SSO 工具，以便更輕鬆地使用 API。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

對於 API 用戶端，有三個類別：`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，`PublicApi` 包含可直接從瀏覽器/行動裝置等無需驗證即可呼叫的 API。`ModerationApi` 包含驅動審核員儀表板的相關方法。

`ModerationApi` 提供廣泛且即時的審核 API。每個 `ModerationApi` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。