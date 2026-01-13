### FastComments 的 API

FastComments 提供一個可與多種資源互動的 API。可用來建立與我們平台的整合，或甚至自行建立客戶端！

在本文件中，您將會找到 API 支援的所有資源，並列出它們的請求與回應類型。

對於企業客戶，所有 API 存取都會被記錄在稽核日誌中。

### 已產生的 SDKs

FastComments 現在會從程式碼產生一份 [API Spec](https://fastcomments.com/js/swagger.json)（尚未完整，但已包含許多 API）。

我們也提供熱門語言的 SDK：

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### 驗證

API 的驗證方式是將您的 [api key](https://fastcomments.com/auth/my-account/api-secret) 以 `X-API-KEY` 標頭或 `API_KEY` 查詢參數傳送。您還需要您的 `tenantId` 來進行 API 呼叫。這可以從與您的 api key 相同的頁面取得。

### 安全注意事項

這些路由應從 **伺服器** 呼叫。 __請勿__ 從瀏覽器呼叫它們。這樣做會暴露您的 API key - 任何能查看頁面原始碼的人都會取得對您帳戶的完全存取權！

#### 驗證選項一 - 標頭

- 標頭: `X-API-KEY`
- 標頭: `X-TENANT-ID`

#### 驗證選項二 - 查詢參數

- 查詢參數: `API_KEY`
- 查詢參數: `tenantId`

---