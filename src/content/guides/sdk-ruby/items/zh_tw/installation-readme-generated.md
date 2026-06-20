將下列這行新增到您應用程式的 Gemfile：

```ruby
gem 'fastcomments'
```

然後執行：

```bash
bundle install
```

或自行安裝：

```bash
gem install fastcomments
```

### 函式庫內容

此函式庫包含產生的 API 用戶端及單點登入 (SSO) 工具，以便更輕鬆地使用 API。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公開與受保護的 API

對於 API 用戶端，有三個類別：`DefaultApi`、`PublicApi` 以及 `ModerationApi`。`DefaultApi` 包含需要您 API 金鑰的方法，而 `PublicApi` 包含可以直接從瀏覽器/行動裝置等發出且不需驗證的 API 呼叫。`ModerationApi` 包含驅動管理者儀表板的方法。

`ModerationApi` 涵蓋評論審核（列出、計數、搜尋、日誌、匯出）、審核操作（移除/復原、檢舉、設定審查/垃圾/核准狀態、投票、重新開啟/關閉討論串）、封鎖（從評論封鎖、取消、封鎖前摘要、封鎖狀態/偏好、被封鎖使用者計數）、以及徽章與信任（授予/移除徽章、手動徽章、取得/設定信任因子、使用者內部檔案）。每個 `ModerationApi` 方法都接受一個 `sso` 參數，這樣請求就可以代表已通過 SSO 驗證的管理員來執行。