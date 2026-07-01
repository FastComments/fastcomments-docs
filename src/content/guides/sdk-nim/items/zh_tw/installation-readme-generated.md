### 使用 Nimble

```bash
nimble install fastcomments
```

### 從原始碼建構

```bash
nimble build
```

### 程式庫內容

此程式庫包含生成的 API 客戶端及 SSO 工具，可讓使用 API 更加便利。

- [API 客戶端函式庫文件](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公開與受保護的 API

對於 API 客戶端，有三個 API 模組，`api_default`、`api_public` 與 `api_moderation`。`api_default` 包含需要您的 API 金鑰的方法，`api_public` 包含可直接從瀏覽器/行動裝置等無需驗證即可呼叫的 API。`api_moderation` 模組則提供給管理員儀表板使用的方法。

`api_moderation` 模組提供廣泛的即時與快速審核 API。每個 `api_moderation` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。