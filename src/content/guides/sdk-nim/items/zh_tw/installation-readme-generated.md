### 使用 Nimble

```bash
nimble install fastcomments
```

### 從原始碼建置

```bash
nimble build
```

### 函式庫內容

此函式庫包含產生的 API 用戶端與 SSO 工具，以便更容易與 API 一起使用。

- [API 用戶端函式庫文件](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公開與受保護的 API

對於 API 用戶端，有三個 API 模組，`api_default`、`api_public` 與 `api_moderation`。`api_default` 包含需要 API 金鑰的方法，`api_public` 則包含可直接從瀏覽器/行動裝置/等進行且無需驗證的 API 呼叫。`api_moderation` 模組包含管理員後台的相關方法。

`api_moderation` 方法涵蓋列出、計數、搜尋與匯出評論及其日誌；審核動作如移除/還原評論、檢舉、設定審查/垃圾/審核狀態、調整投票，以及重新開啟/關閉討論串；封鎖相關功能（封鎖使用者留言、取消封鎖、封鎖前摘要、封鎖狀態與偏好設定，以及被封鎖使用者數量）；與徽章與信任（授予/移除徽章、列出手動徽章、取得/設定使用者的信任係數，以及擷取使用者的內部檔案）。每個 `api_moderation` 方法都接受一個 `sso` 參數，以便該呼叫以 SSO 管理員身份進行驗證。