### 使用已驗證的 API (DefaultAPI)

**重要：**
1. 您必須設定 base URL（cpp-restsdk generator 不會從 OpenAPI spec 中讀取它）
2. 您必須在 ApiClient 上設定您的 API key 才能進行已驗證的請求。如果不這麼做，請求會以 401 錯誤失敗。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須：設定 base URL（選擇您的區域）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // 或： config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必須：設定您的 API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 現在開始進行已驗證的 API 呼叫
    return 0;
}
```

### 使用公開 API (PublicAPI)

公開端點不需要驗證：

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須：設定 base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 進行公開 API 呼叫
    return 0;
}
```

### 常見問題

1. **"URI must contain a hostname" error**：請確保在建立 ApiClient 之前呼叫 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`。cpp-restsdk generator 不會自動從 OpenAPI spec 中讀取伺服器 URL。
2. **401 "missing-api-key" error**：請確保在建立 DefaultAPI 實例之前呼叫 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`。
3. **Wrong API class**：對於伺服端已驗證的請求，請使用 `DefaultAPI`；對於用戶端/公開請求，請使用 `PublicAPI`。