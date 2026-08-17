### 從 GitHub 安裝

直接從發行標籤安裝（推薦，完全可重現）：

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

將標籤固定（pin）而非分支，以確保建置具決定性。相同的寫法也適用於 `requirements.txt`：

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

每個已標記的 [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) 也附有已建好的 wheel，如果你想直接安裝二進位制檔案的話。

### 函式庫內容

此函式庫包含兩個模組：產生的 API 客戶端以及核心 Python 函式庫，後者包含手寫的工具函式，以簡化與 API 的互動，並支援 SSO。

- [API 客戶端函式庫文件](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心函式庫文件，包含 SSO 範例](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開與受保護的 API

對於 API 客戶端，有三個類別，`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要 API 金鑰的方法，`PublicApi` 包含可直接從瀏覽器/行動裝置等發出且不需驗證的方法。`ModerationApi` 提供廣泛的即時與快速審核 API。每個 `ModerationApi` 方法皆接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。