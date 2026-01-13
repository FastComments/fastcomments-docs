### Коришћење аутентификованих API-ја (DefaultAPI)

**Важно:**
1. Морате поставити базну URL адресу (cpp-restsdk generator не чита га из OpenAPI spec)
2. Морате поставити ваш API кључ на ApiClient пре слања аутентификованих захтева. Ако то не урадите, захтеви ће се неуспешно завршити са 401 грешком.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Поставите базну URL адресу (изаберите ваш регион)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ИЛИ: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ОБАВЕЗНО: Поставите ваш API кључ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Сада правите аутентификоване API позиве
    return 0;
}
```

### Коришћење јавних API-ја (PublicAPI)

Јавни endpoints не захтевају аутентификацију:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Поставите базну URL адресу
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Правите јавне API позиве
    return 0;
}
```

### Чести проблеми

1. **"URI must contain a hostname" error**: Уверите се да позовете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` пре него што креирате ApiClient. The cpp-restsdk generator doesn't automatically read the server URL from the OpenAPI spec.
2. **401 "missing-api-key" error**: Уверите се да позовете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` пре него што креирате DefaultAPI инстанцу.
3. **Wrong API class**: Користите `DefaultAPI` за серверске аутентификоване захтеве, `PublicAPI` за клијентске/јавне захтеве.