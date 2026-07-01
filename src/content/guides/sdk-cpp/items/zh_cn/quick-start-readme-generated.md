### 使用已认证的 API（DefaultAPI）

**重要提示：**
1. 您必须设置基础 URL（cpp-restsdk 生成器不会从 OpenAPI 规范中读取它）
2. 您必须在发出已认证请求之前在 ApiClient 上设置您的 API 密钥。如果不设置，请求将会返回 401 错误。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必需：设置基础 URL（选择您的地区）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // 或者: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必需：设置您的 API 密钥
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 现在发出已认证的 API 调用
    return 0;
}
```

### 使用公共 API（PublicAPI）

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

    // 发出公共 API 调用
    return 0;
}
```

### 使用审核 API（ModerationApi）

`ModerationApi` 为审核员仪表板提供功能。每个方法都接受 `sso` 参数，以便代表已通过 SSO 认证的审核员执行调用（请参阅下文 SSO 部分了解如何创建令牌）：

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

    // 将审核员的 SSO 令牌传入以进行身份验证
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### 常见问题

1. **“URI 必须包含主机名”错误**：确保在创建 ApiClient 之前调用 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`。cpp-restsdk 生成器不会自动从 OpenAPI 规范读取服务器 URL。  
2. **401 “missing-api-key” 错误**：确保在创建 DefaultAPI 实例之前调用 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`。  
3. **错误的 API 类**：对服务器端已认证请求使用 `DefaultApi`，对客户端/公共请求使用 `PublicApi`，对审核员仪表板请求（使用审核员 SSO 令牌认证）使用 `ModerationApi`。