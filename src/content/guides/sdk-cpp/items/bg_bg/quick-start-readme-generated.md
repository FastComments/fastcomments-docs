### Използване на удостоверени API (DefaultAPI)

**Важно:**
1. Трябва да зададете базовия URL (генераторът cpp-restsdk не го чете от OpenAPI спецификацията)
2. Трябва да зададете своя API ключ в ApiClient преди да правите удостоверени заявки. Ако не го направите, заявките ще се провалят с грешка 401.

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

    // ЗАДЪЛЖИТЕЛНО: Задайте своя API ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Сега направете удостоверени API извиквания
    return 0;
}
```

### Използване на публични API (PublicAPI)

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

    // Правете публични API извиквания
    return 0;
}
```

### Използване на Moderation API (ModerationApi)

`ModerationApi` захранва таблото за модерация. Всеки метод приема параметър `sso`, така че извикването се изпълнява от името на модератор, удостоверен чрез SSO (вижте раздела SSO по-долу за това как да създадете токен):

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

    // Предайте SSO токена на модератора, за да удостоверите заявката
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

1. **Грешка "URI must contain a hostname"**: Уверете се, че извиквате `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` преди да създадете ApiClient. Генераторът cpp-restsdk не чете автоматично URL на сървъра от OpenAPI спецификацията.
2. **401 "missing-api-key" грешка**: Уверете се, че извиквате `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` преди да създадете DefaultAPI инстанция.
3. **Неправилен клас на API**: Използвайте `DefaultApi` за удостоверени заявки от страна на сървъра, `PublicApi` за заявки от страна на клиента/публични заявки и `ModerationApi` за заявки към таблото за модерация (удостоверени с SSO токен на модератор).
---