### 使用 Nimble

```bash
nimble install fastcomments
```

### 從原始碼建置

```bash
nimble build
```

### 函式庫內容

此函式庫包含產生的 API 用戶端以及 SSO 工具，以便更輕鬆地使用 API。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公開與受保護的 API

For the API client, there are two API modules, `api_default` and `api_public`. The `api_default` contains methods that require your API key, and `api_public` contains api calls
而 `api_public` 則包含 API 呼叫
可直接從瀏覽器／行動裝置等進行，且無需驗證。