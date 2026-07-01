### 安裝相依套件

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### 從原始碼建置

```bash
mkdir build
cd build
cmake ..
make
```

### 安裝

```bash
sudo make install
```

### 函式庫內容

此函式庫包含已產生的 API 客戶端以及 SSO 工具，可讓使用 API 更加方便。

- [API 客戶端函式庫文件](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公開 API 與受保護 API

對於 API 客戶端，有三個類別，`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，`PublicApi` 包含可直接從瀏覽器／行動裝置等無需驗證即可呼叫的方法。`ModerationApi` 提供大量即時且快速的審核 API。每個 `ModerationApi` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。