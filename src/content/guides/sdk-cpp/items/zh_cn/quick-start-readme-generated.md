### 使用已认证的 API (DefaultAPI)

**重要：**
1. 您必须设置基础 URL（cpp-restsdk 生成器不会从 OpenAPI 规范中读取它）
2. 在进行需要认证的请求之前，您必须在 ApiClient 上设置您的 API 密钥。如果不设置，请求将返回 401 错误。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必需：设置基础 URL（选择您的区域）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必需：设置您的 API 密钥
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 现在执行已认证的 API 调用
    return 0;
}
```

### 使用公共 API (PublicAPI)

公共端点不需要认证：

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

    // 执行公共 API 调用
    return 0;
}
```

### 使用审核 API (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必需：设置基础 URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // 传入版主的 SSO 令牌以对调用进行认证
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

### 常见问题

1. **"URI must contain a hostname" 错误**：确保在创建 ApiClient 之前调用 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`。cpp-restsdk 生成器不会自动从 OpenAPI 规范读取服务器 URL。
2. **401 "missing-api-key" 错误**：确保在创建 DefaultAPI 实例之前调用 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`。
3. **错误的 API 类**：对于服务器端的已认证请求使用 `DefaultApi`，对于客户端/公共请求使用 `PublicApi`，对于版主仪表板请求使用 `ModerationApi`（使用版主的 SSO 令牌进行认证）。
---