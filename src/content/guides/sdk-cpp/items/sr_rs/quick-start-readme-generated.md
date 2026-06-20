### Коришћење аутентификованих API-ја (DefaultAPI)

**Важно:**
1. Морате подесити базни URL (cpp-restsdk генератор не чита га из OpenAPI спецификације)
2. Морате поставити ваш API кључ на ApiClient пре него што пошаљете аутентификоване захтеве. Ако то не урадите, захтеви ће пропасти са грешком 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Подесите базни URL (изаберите вашу регију)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // САД
    // ИЛИ: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // ЕУ

    // ОБАВЕЗНО: Подесите ваш API кључ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Сада направите аутентификоване API позиве
    return 0;
}
```

### Коришћење јавних API-ја (PublicAPI)

Јавни крајњи ендпоинти не захтевају аутентификацију:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Подесите базни URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Направите јавне API позиве
    return 0;
}
```

### Коришћење модерацијских API-ја (ModerationApi)

`ModerationApi` покреће контролну таблу модератора. Свака метода прихвата параметар `sso` тако да позив ради у име модератора аутентификованог преко SSO (погледајте одељак SSO испод за информације о томе како направити токен):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБАВЕЗНО: Подесите базни URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Проследите SSO токен модератора да бисте аутентификовали позив
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

### Чести проблеми

1. **"URI must contain a hostname" error**: Убедите се да позовете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` пре него што креирате ApiClient. Генератор cpp-restsdk не чита аутоматски URL сервера из OpenAPI спецификације.
2. **401 "missing-api-key" error**: Убедите се да позовете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` пре него што креирате DefaultAPI инстанцу.
3. **Wrong API class**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве контролне табле модератора (аутентификовано помоћу SSO токена модератора).
---