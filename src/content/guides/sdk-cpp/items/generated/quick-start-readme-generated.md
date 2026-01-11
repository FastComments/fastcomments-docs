### Using Authenticated APIs (DefaultAPI)

**Important:**
1. You must set the base URL (cpp-restsdk generator doesn't read it from the OpenAPI spec)
2. You must set your API key on the ApiClient before making authenticated requests. If you don't, requests will fail with a 401 error.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL (choose your region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: Set your API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### Using Public APIs (PublicAPI)

Public endpoints don't require authentication:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Make public API calls
    return 0;
}
```

### Common Issues

1. **"URI must contain a hostname" error**: Make sure you call `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` before creating the ApiClient. The cpp-restsdk generator doesn't automatically read the server URL from the OpenAPI spec.
2. **401 "missing-api-key" error**: Make sure you call `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` before creating the DefaultAPI instance.
3. **Wrong API class**: Use `DefaultAPI` for server-side authenticated requests, `PublicAPI` for client-side/public requests.