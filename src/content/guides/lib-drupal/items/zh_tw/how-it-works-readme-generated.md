當使用者造訪啟用了 FastComments 欄位的實體時：

1. FastComments 的 JavaScript 小工具會從 CDN 載入。
2. 如果已設定 SSO，使用者的 Drupal 身分會傳送給 FastComments。
3. 一個 `<noscript>` 回退機制會為沒有 JavaScript 的使用者提供伺服器端渲染的留言（僅限 Live Comments 與 Streaming Chat 模式）。