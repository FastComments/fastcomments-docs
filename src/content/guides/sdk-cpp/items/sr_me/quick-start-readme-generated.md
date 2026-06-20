### Коришћење аутентификованих API-ја (DefaultAPI)

**Важно:**
1. Морате да подесите базни URL (cpp-restsdk generator га не чита из OpenAPI спецификације)
2. Морате да подесите ваш API кључ на ApiClient пре него што направите аутентификоване захтеве. Ако то не урадите, захтеви ће бити одбијени са 401 грешком.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ПОТРЕБНО: Подесите базни URL (изаберите вашу регију)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ПОТРЕБНО: Подесите ваш API кључ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Сада направите аутентификоване API позиве
    return 0;
}
```

### Коришћење јавних API-ја (PublicAPI)

Јавни ентпоинти не захтевају аутентификацију:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ПОТРЕБНО: Подесите базни URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Направите јавне API позиве
    return 0;
}
```

### Коришћење модерацијских API-ја (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Сва метода прихвата параметар `sso` тако да позив ради у име модератора аутентификованог преко SSO (погледајте одељак о SSO-у испод за упутство како креирати токен):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ПОТРЕБНО: Подесите базни URL
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

1. **"URI must contain a hostname" error**: Уверите се да позивате `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` пре него што креирате ApiClient. cpp-restsdk generator аутоматски не чита сервер URL из OpenAPI спецификације.
2. **401 "missing-api-key" error**: Уверите се да позивате `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` пре него што креирате DefaultAPI инстанцу.
3. **Wrong API class**: Користите `DefaultApi` за сервер-сајд аутентификоване захтеве, `PublicApi` за клијент-сајд/јавне захтеве, и `ModerationApi` за захтеве контролне табле модератора (аутентификовано помоћу SSO токена модератора).