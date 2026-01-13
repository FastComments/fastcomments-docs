### Коришћење аутентификованих API-ја (DefaultAPI)

**Важно:**
1. Морате поставити базни URL (cpp-restsdk генератор не чита то из OpenAPI спецификације)
2. Морате поставити ваш API кључ на ApiClient прије слања аутентификованих захтјева. Ако не, захтјеви ће бити одбијени са грешком 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Поставите базни URL (изаберите вашу регију)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ИЛИ: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ОБАВЕЗНО: Поставите ваш API кључ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### Коришћење јавних API-ја (PublicAPI)

Јавни ендпоинти не захтијевају аутентификацију:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Поставите базни URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Извршите јавне API позиве
    return 0;
}
```

### Чести проблеми

1. **"URI must contain a hostname" error**: Убедите се да позовете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` прије креирања ApiClient-а. cpp-restsdk генератор не чита аутоматски URL сервера из OpenAPI спецификације.
2. **401 "missing-api-key" error**: Убедите се да позовете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` прије креирања DefaultAPI инстанце.
3. **Wrong API class**: Користите `DefaultAPI` за серверске аутентификоване захтјеве, `PublicAPI` за клијентске/јавне захтјеве.