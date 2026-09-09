### FastComments API

FastComments 提供一套 API 以與多種資源互動。您可以與我們的平台整合，甚至自行開發客戶端！

在本文件中，您將找到 API 所支援的所有資源，並附有其請求與回應類型的說明。

對於企業客戶，所有 API 存取皆會記錄於稽核日誌。

### 產生的 SDK

FastComments 現在會從我們的程式碼產生 [API Spec](https://fastcomments.com/js/swagger.json)（此規格尚未完整，但已包含許多 API）。

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

### 認證

API 透過傳遞您的 [api key](https://fastcomments.com/auth/my-account/api-secret) 作為 `X-API-KEY` 標頭或 `API_KEY` 查詢參數來驗證。您還需要 `tenantId` 以進行 API 呼叫。此資訊可在與 api key 同一頁面取得。

### 安全說明

這些路由應該從 **伺服器** 端呼叫。__請勿__ 從瀏覽器呼叫。這樣會暴露您的 API 金鑰，讓任何能看到頁面原始碼的人取得您帳號的完整存取權限！

#### 認證選項一 - 標頭

- 標頭: `X-API-KEY`
- 標頭: `X-TENANT-ID`

#### 選項二 - 查詢參數

- 查詢參數: `API_KEY`
- 查詢參數: `tenantId`

#### 選項三 - OAuth Bearer Token

- 標頭: `Authorization: Bearer fcat_...`

透過 [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) 連線的應用程式會以 OAuth 取得 token，而非使用 API 金鑰。此 token 可用於此處的所有端點。租戶資訊由 token 隱含，因此 `tenantId` 為可選項，但若提供必須與 token 相符。`GET` 請求需要 `read` 範圍，其他方法則需要 `write` 範圍。發現流程起始於 `https://fastcomments.com/.well-known/oauth-authorization-server`。

### 讀取自己的寫入

FastComments 提供 Active-Active 可用性。來自您資料中心的請求會被導向至離您最近的 [點位](https://sophon.fastcomments.com/)。此過程自動完成，通常您即可觀察到「寫入後即讀」的語意。若您想確保讀取自己的寫入，可透過將請求固定到特定區域的 API 主機來實現（但對大多數整合而言通常不需要）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

請注意，若採用此方式，您可能需要設定備援，因為我們過去已棄用某些入口節點，且在切換時會使用新名稱。