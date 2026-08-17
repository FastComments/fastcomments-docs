### 從 GitHub 安裝

直接從發行標籤安裝（推薦，完全可重現）：

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

將標籤固定而非分支，以確保建置具決定性。相同的寫法也適用於 `requirements.txt`：

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

每個標記的 [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) 也附有已建好的 wheel，若您想直接安裝二進位套件，可使用此方式。

### 函式庫內容

此函式庫包含兩個模組：產生的 API 客戶端以及核心 Python 函式庫，後者包含手寫的工具函式，以讓使用 API 更加便利，亦支援 SSO。

- [API 客戶端函式庫文件](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心函式庫文件，含 SSO 範例](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開與受保護的 API

對於 API 客戶端，有三個類別，`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，`PublicApi` 包含可直接從瀏覽器/行動裝置等無需驗證即可呼叫的方法。`ModerationApi` 提供廣泛且快速的即時審核 API。每個 `ModerationApi` 方法皆接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。