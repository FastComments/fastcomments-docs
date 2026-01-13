### Използване на удостоверени API-та (DefaultAPI)

**Важно:**
1. Трябва да зададете базовия URL (cpp-restsdk генераторът не го чете от OpenAPI спецификацията)
2. Трябва да зададете вашия API ключ на ApiClient преди да правите удостоверени заявки. Ако не го направите, заявките ще върнат грешка 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ЗАДЪЛЖИТЕЛНО: Задайте базовия URL (изберете вашия регион)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Сега извършете удостоверени повиквания към API-то
    return 0;
}
```

### Използване на публични API-та (PublicAPI)

Публичните крайни точки не изискват удостоверяване:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ЗАДЪЛЖИТЕЛНО: Задайте базовия URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Извършете публични повиквания към API-то
    return 0;
}
```

### Общи проблеми

1. **"URI must contain a hostname" error**: Уверете се, че извиквате `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` преди да създадете ApiClient. cpp-restsdk генераторът не чете автоматично URL-а на сървъра от OpenAPI спецификацията.
2. **401 "missing-api-key" error**: Уверете се, че извиквате `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` преди да създадете инстанция на DefaultAPI.
3. **Wrong API class**: Използвайте `DefaultAPI` за удостоверени заявки от страна на сървъра, `PublicAPI` за клиентски/публични заявки.