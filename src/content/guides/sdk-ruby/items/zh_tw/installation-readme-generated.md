---
將這一行加入您應用程式的 Gemfile：

```ruby
gem 'fastcomments'
```

然後執行：

```bash
bundle install
```

或使用下列指令自行安裝：

```bash
gem install fastcomments
```

### 函式庫內容

此函式庫包含已產生的 API 用戶端與 SSO 工具，能使使用 API 更加簡便。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公開與受保護的 API

對於 API 用戶端，有兩個類別，`DefaultApi` 與 `PublicApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，而 `PublicApi` 包含可直接從瀏覽器/行動裝置等
在不需驗證的情況下呼叫的 API。
---