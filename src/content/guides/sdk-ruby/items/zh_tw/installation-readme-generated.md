在你的應用程式的 Gemfile 中加入這一行：

```ruby
gem 'fastcomments'
```

然後執行：

```bash
bundle install
```

或者自行安裝：

```bash
gem install fastcomments
```

### 函式庫內容

此函式庫包含自動產生的 API 用戶端以及 SSO 工具，讓使用 API 更加容易。

- [API 用戶端函式庫文件](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公開 API 與受保護 API

API 用戶端包含三個類別:`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的方法,而 `PublicApi` 包含可以直接從瀏覽器/行動裝置等進行呼叫、無需驗證的 API。`ModerationApi` 包含驅動審核儀表板的方法。

`ModerationApi` 提供一套廣泛的即時快速審核 API。每個 `ModerationApi` 方法都接受 `sso` 參數,並可透過 SSO 或 FastComments.com 的 session cookie 進行驗證。