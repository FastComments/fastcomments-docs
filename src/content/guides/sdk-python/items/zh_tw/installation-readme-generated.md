### 從 GitHub 安裝

直接從發布標籤安裝（推薦，完全可重現）：

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

將標籤固定而不是分支，以確保構建具有決定性。同樣的寫法也可用於 `requirements.txt`：

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

每個帶標籤的 [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) 也附有已編譯的 wheel，如果你想直接安裝二進位制套件，可使用它。

### 程式庫內容

此程式庫包含兩個模組：產生的 API 客戶端以及包含手寫工具函式的核心 Python 程式庫，讓使用 API 更加便利，並支援 SSO。

- [API 客戶端程式庫文件](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [核心程式庫文件，含 SSO 範例](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開與保護 API

對於 API 客戶端，有三個類別，`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要 API 金鑰的方法，`PublicApi` 包含可直接從瀏覽器／行動裝置等無需驗證即可呼叫的方法。`ModerationApi` 提供完整且快速的即時審查 API 套件。每個 `ModerationApi` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 cookie 進行驗證。