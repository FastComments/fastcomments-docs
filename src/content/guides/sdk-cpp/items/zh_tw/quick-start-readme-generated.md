### 使用已驗證的 API (DefaultAPI)

**重要：**
1. 您必須設定 base URL（cpp-restsdk generator 不會從 OpenAPI spec 中讀取它）
2. 您必須在進行已驗證的請求之前在 ApiClient 上設定您的 API 金鑰。如果不這麼做，請求會以 401 錯誤失敗。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必填：設定 base URL（選擇您的區域）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // 或：config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必填：設定您的 API 金鑰
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 現在進行已驗證的 API 呼叫
    return 0;
}
```

### 使用公開的 API (PublicAPI)

公開端點不需要驗證：

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必填：設定 base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 進行公開 API 呼叫
    return 0;
}
```

### 使用 Moderation APIs (ModerationApi)

`ModerationApi` 為版主儀表板提供功能。每個方法都接受一個 `sso` 參數，使呼叫以 SSO 驗證的版主身份執行（請參閱下方的 SSO 區段，說明如何建立一個 token）：

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必填：設定 base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // 傳遞版主的 SSO 令牌以驗證該呼叫
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    auto response = moderationApi.getCount(
        boost::none,  // textSearch
        boost::none,  // byIPFromComment
        boost::none,  // filter
        boost::none,  // searchFilters
        boost::none,  // demo
        ssoToken      // sso
    ).get();

    return 0;
}
```

### 常見問題

1. **"URI must contain a hostname" error**：確保在建立 ApiClient 之前呼叫 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`。cpp-restsdk generator 不會自動從 OpenAPI spec 讀取 server URL。
2. **401 "missing-api-key" error**：確保在建立 DefaultAPI 實例之前呼叫 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`。
3. **Wrong API class**：對於伺服器端的已驗證請求使用 `DefaultApi`，對於客戶端/公開請求使用 `PublicApi`，對於版主儀表板請求（使用版主 SSO 令牌驗證）使用 `ModerationApi`。