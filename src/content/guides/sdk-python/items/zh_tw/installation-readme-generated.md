### PyPI

```bash
pip install fastcomments
```

### 函式庫內容

此函式庫包含兩個模組：產生的 API 用戶端，以及包含手寫工具的核心 Python 函式庫，這些工具可讓使用 API 更加方便，包括 SSO 支援。

- [API 用戶端函式庫文件](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心函式庫文件，包含 SSO 範例](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開與受保護的 API

對於 API 用戶端，包含三個類別：`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要 API 金鑰的方法，`PublicApi` 則包含可直接從瀏覽器、行動裝置等在未驗證情況下呼叫的方法。`ModerationApi` 提供管理者儀表板的功能，包含用於管理留言的方法（列出、計數、搜尋、日誌、匯出）、審核動作（移除/還原、標記、設定審查/垃圾/核准狀態、投票、重新開啟/關閉討論串）、封禁相關功能（對留言封禁、復原、預先封禁摘要、封禁狀態/偏好、被封禁使用者數量）、以及徽章與信任（授予/移除徽章、手動徽章、取得/設定信任因子、使用者內部資料）。每個 `ModerationApi` 方法都接受一個 `sso` 參數，允許以經 SSO 驗證的管理者身分呼叫。