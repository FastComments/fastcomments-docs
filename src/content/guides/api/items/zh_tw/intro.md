### The FastComments API

FastComments 提供一個用於互動多種資源的 API。可用它來建立與我們平台的整合，或甚至自行建立客戶端！

在本文件中，您會找到 API 支援的所有資源，以及它們的請求與回應型別的文件。

對於企業客戶，所有的 API 存取都會被記錄在稽核日誌 (Audit Log) 中。

### Generated SDKs

FastComments 現在會從我們的程式碼產生一份 [API Spec](https://fastcomments.com/js/swagger.json)（尚未完全，但包含許多 API）。

我們也為熱門語言提供了 SDK：

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

### Authentication

API 的驗證方式是將您的 [API 金鑰](https://fastcomments.com/auth/my-account/api-secret) 作為 `X-API-KEY` 標頭或 `API_KEY` 查詢參數傳遞。您也需要您的 `tenantId` 來進行 API 呼叫。這可以從與您的 API 金鑰相同的頁面取得。

### Security Note

這些路由預期由一個 **伺服器** 呼叫。__DO NOT__ 從瀏覽器呼叫它們。這樣做會暴露您的 API 金鑰——任何能查看頁面原始碼的人都能取得完整的帳戶存取權！

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Reading Your Own Writes

FastComments 提供 Active-Active 可用性。來自您資料中心的請求會被導向距離您最近的 [point of presence](https://sophon.fastcomments.com/)。這是自動的，通常您可以觀察到「讀到自己的寫入」語意。如果您想確保能讀到自己的寫入，您可以透過使用該區域作為 API 主機來將請求鎖定至某個區域（不過對於大多數整合來說通常不需要）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

請注意，如果您這樣做，您可能需要定義一個回退機制，因為我們過去曾棄用入口節點並在切換時使用新的名稱。