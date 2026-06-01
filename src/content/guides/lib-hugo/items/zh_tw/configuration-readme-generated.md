所有 FastComments 小工具選項都在 `hugo.toml` 的 `[params.fastcomments]` 下設定，並可在每頁的 front matter 中的 `[fastcomments]` 覆蓋。優先順序（由低到高）：網站參數、頁面 front matter、短碼參數。

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# 該頁面的 front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

當既未提供 `url` 也未提供 `urlId` 時，`url` 預設為該頁面的 permalink，以便留言串綁定到穩定的 URL。

### 歐盟資料駐留

歐盟客戶將 `region = "eu"` 設定以將小工具導向 `cdn-eu.fastcomments.com`：

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### 鍵大小寫說明

Hugo 會將 `hugo.toml` 和 front matter 中的所有鍵轉為小寫，但 FastComments 小工具需要 camelCase 鍵（如 `tenantId`、`hasDarkBackground`）。此元件會自動還原每個已知頂層選項的正確大小寫，因此請以正常的 camelCase 形式撰寫選項。位於物件值內的巢狀鍵（例如 `translations` 映射的鍵，或 `pageReactConfig` 的欄位）不會被還原。請改透過 FastComments 控制台的自訂化 UI 來設定那些。