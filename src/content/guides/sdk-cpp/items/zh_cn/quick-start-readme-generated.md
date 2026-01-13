### 使用已认证的 API (DefaultAPI)

**重要：**
1. 您必须设置基础 URL（cpp-restsdk 生成器不会从 OpenAPI 规范中读取它）
2. 在发出已认证请求之前，您必须在 ApiClient 上设置您的 API 密钥。如果不设置，请求将以 401 错误失败。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必需：设置基础 URL（选择您的区域）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // 美国
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // 欧盟

    // 必需：设置您的 API 密钥
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 现在开始发出已认证的 API 调用
    return 0;
}
```

### 使用公共 API (PublicAPI)

公共端点不需要身份验证：

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必需：设置基础 URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 发出公共 API 调用
    return 0;
}
```

### 常见问题

1. **“URI must contain a hostname” 错误**：确保在创建 ApiClient 之前调用 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`。cpp-restsdk 生成器不会自动从 OpenAPI 规范中读取服务器 URL。
2. **401 “missing-api-key” 错误**：确保在创建 DefaultAPI 实例之前调用 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`。
3. **使用了错误的 API 类**：对服务器端的已认证请求使用 `DefaultAPI`，对客户端/公共请求使用 `PublicAPI`。