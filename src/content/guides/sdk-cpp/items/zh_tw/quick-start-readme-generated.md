### 使用已驗證的 API（DefaultAPI）

**重要：**
1. 必須設定基礎 URL（cpp-restsdk 產生器不會從 OpenAPI 規格中讀取它）
2. 必須在發出已驗證請求之前於 ApiClient 上設定您的 API 金鑰。若未設定，請求將會以 401 錯誤失敗。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必填：設定基礎 URL（選擇您的區域）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必填：設定您的 API 金鑰
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 現在執行已驗證的 API 呼叫
    return 0;
}
```

### 使用公共 API（PublicAPI）

公共端點不需要驗證：

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必填：設定基礎 URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 執行公共 API 呼叫
    return 0;
}
```

### 使用審核 API（ModerationApi）

`ModerationApi` 為審核員儀表板提供功能。每個方法都接受 `sso` 參數，使呼叫以已透過 SSO 驗證的審核員身分執行（請參閱下方 SSO 章節了解如何建立 token）：

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必填：設定基礎 URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // 傳遞審核員的 SSO token 以驗證此呼叫
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### 常見問題

1. **「URI 必須包含主機名稱」錯誤**：確保在建立 ApiClient 前呼叫 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`。cpp-restsdk 產生器不會自動從 OpenAPI 規格讀取伺服器 URL。  
2. **401 「missing-api-key」錯誤**：確保在建立 DefaultAPI 實例前呼叫 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`。  
3. **API 類別錯誤**：對於伺服器端已驗證請求使用 `DefaultApi`，對於客戶端/公共請求使用 `PublicApi`，對於審核員儀表板請求（以審核員 SSO token 驗證）使用 `ModerationApi`。