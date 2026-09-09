### The FastComments API

FastComments 提供一套 API 以與多種資源互動。您可以與我們的平台整合，甚至自行開發客戶端！

在本文件中，您將找到 API 支援的所有資源，並附有其請求與回應類型的說明。

對於企業客戶，所有 API 存取皆會記錄於稽核日誌中。

### Generated SDKs

FastComments 現在會從我們的程式碼產生 [API Spec](https://fastcomments.com/js/swagger.json) (尚未完整，但已包含許多 API)。

我們也提供了多種流行語言的 SDK：

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

API 透過傳遞您的 [api key](https://fastcomments.com/auth/my-account/api-secret) 進行驗證，可使用 `X-API-KEY` 標頭或 `API_KEY` 查詢參數。您還需要 `tenantId` 以呼叫 API。此資訊可在與 API 金鑰相同的頁面取得。

### Security Note

這些路由應該由 **伺服器** 呼叫。__絕不可__ 從瀏覽器呼叫。這樣會暴露您的 API 金鑰，任何能看到頁面原始碼的人都能取得您帳號的完整存取權！

#### Authentication Option One - Headers

- 標頭：`X-API-KEY`
- 標頭：`X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- 查詢參數：`API_KEY`
- 查詢參數：`tenantId`

#### Authentication Option Three - OAuth Bearer Token

- 標頭：`Authorization: Bearer fcat_...`

透過 [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) 連線的應用程式會以 OAuth 取得 token，而非使用 API 金鑰。此 token 可用於此處的所有端點。租戶資訊由 token 隱含，因此 `tenantId` 為可選項目，但若提供必須與 token 相符。`GET` 請求需要 `read` 範圍，其他方法則需要 `write` 範圍。發現流程起始於 `https://fastcomments.com/.well-known/oauth-authorization-server`。

### Reading Your Own Writes

FastComments 提供 Active-Active 可用性。來自您資料中心的請求會被導向離您最近的節點。此過程自動完成，通常您可以觀察到讀寫一致性（read‑your‑write）語意。若您想確保讀到自己的寫入，可將請求固定到特定區域，方法是使用該區域的 API 主機（但對大多數整合而言通常不需要這麼做）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

請注意，若您這樣做，可能需要設定備援，因為我們過去已棄用某些入口節點，並使用新名稱進行切換。