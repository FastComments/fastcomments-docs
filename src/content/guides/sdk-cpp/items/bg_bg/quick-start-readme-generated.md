### Използване на автентикирани API (DefaultAPI)

**Важно:**
1. Трябва да зададете базовия URL (cpp-restsdk генераторът не го чете от OpenAPI спецификацията)
2. Трябва да зададете вашия API ключ на ApiClient, преди да правите автентикирани заявки. Ако не го направите, заявките ще се провалят с грешка 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ЗАДЪЛЖИТЕЛНО: Задайте базовия URL (изберете вашия регион)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ИЛИ: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ЗАДЪЛЖИТЕЛНО: Задайте вашия API ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Сега правете автентикирани API извиквания
    return 0;
}
```

### Използване на публични API (PublicAPI)

Публичните крайни точки не изискват автентикация:

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

    // Правете публични API извиквания
    return 0;
}
```

### Използване на API за модериране (ModerationApi)

`ModerationApi` захранва таблото за модериране. Всеки метод приема параметър `sso`, така че извикването да се изпълни от името на SSO-автентикиран модератор (вижте секцията за SSO по-долу
за това как да създадете токен):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ЗАДЪЛЖИТЕЛНО: Задайте базовия URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Подайте SSO токена на модератора, за да автентикирате извикването
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Чести проблеми

1. **Грешка "URI must contain a hostname"**: Уверете се, че извиквате `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` преди създаването на ApiClient. Генераторът cpp-restsdk не чете автоматично URL адреса на сървъра от OpenAPI спецификацията.
2. **Грешка 401 "missing-api-key"**: Уверете се, че извиквате `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` преди създаването на DefaultAPI инстанцията.
3. **Грешен API клас**: Използвайте `DefaultApi` за сървърни автентикирани заявки, `PublicApi` за клиентски/публични заявки и `ModerationApi` за заявки към таблото за модериране (автентикирани със SSO токен на модератор).