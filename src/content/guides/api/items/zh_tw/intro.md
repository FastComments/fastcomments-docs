### FastComments API

FastComments 提供一套 API 讓您與多種資源互動。您可以使用我們的平台建置整合，甚至自行開發客戶端！

在本文件中，您將找到 API 所支援的所有資源，並附有其請求與回應類型的說明。

對於企業客戶，所有 API 存取皆會記錄於稽核日誌中。

### 產生的 SDK

FastComments 現在會從程式碼產生一份 [API Spec](https://fastcomments.com/js/swagger.json)（此規格尚未完整，但已包含許多 API）。

我們也提供了多種常見語言的 SDK：

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

API 透過在 `X-API-KEY` 標頭或 `API_KEY` 查詢參數中傳遞您的 [api key](https://fastcomments.com/auth/my-account/api-secret) 來驗證。您還需要提供 `tenantId` 以呼叫 API。`tenantId` 可從與 API 金鑰相同的頁面取得。

### 安全說明

這些路由應該由 **伺服器** 呼叫。**請勿** 從瀏覽器呼叫。若這樣做會暴露您的 API 金鑰，任何能看到頁面原始碼的人都能取得完整帳號存取權限！

#### 認證方式一 – 標頭

- 標頭：`X-API-KEY`
- 標頭：`X-TENANT-ID`

#### 認證方式二 – 查詢參數

- 查詢參數：`API_KEY`
- 查詢參數：`tenantId`

#### 認證方式三 – OAuth Bearer Token

- 標頭：`Authorization: Bearer fcat_...`

第三方應用程式（如 Zapier）以及 [MCP 伺服器](https://docs.fastcomments.com/guide-llm-kit.html) 的客戶端會透過 OAuth 取得 token，而非使用 API 金鑰。此 token 可用於此處的所有端點。token 已隱含租戶資訊，故 `tenantId` 為可選，但若提供則必須與 token 相符。`GET` 請求需要 `read` 範圍，其他方法則需要 `write` 範圍。完整流程（包括客戶端註冊、PKCE、刷新與撤銷）請參考 [OAuth Authorization](#oauth)。發現端點位於 `https://fastcomments.com/.well-known/oauth-authorization-server`。

### 讀取自己的寫入

FastComments 提供 Active-Active 可用性。來自您資料中心的請求會自動路由至離您最近的 [presence 點](https://sophon.fastcomments.com/)。這是自動化的，通常您可以觀察到讀寫一致性（read‑your‑write）語意。若您想確保讀取自己的寫入，可將請求固定到特定區域的 API 主機（但大多數整合通常不需要這麼做）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

請注意，若您這樣做，可能需要設定備援，因為我們過去已棄用某些入口節點，並使用新名稱進行切換。