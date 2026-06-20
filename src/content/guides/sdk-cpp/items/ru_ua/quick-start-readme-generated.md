### Using Authenticated APIs (DefaultAPI)

**Важно:**
1. Вы должны установить базовый URL (генератор cpp-restsdk не читает его из спецификации OpenAPI)
2. Вы должны установить ваш API-ключ в ApiClient перед выполнением аутентифицированных запросов. Если этого не сделать, запросы завершатся ошибкой 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБЯЗАТЕЛЬНО: Установите базовый URL (выберите ваш регион)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ОБЯЗАТЕЛЬНО: Установите ваш API-ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Теперь выполняйте аутентифицированные вызовы API
    return 0;
}
```

### Using Public APIs (PublicAPI)

Публичные конечные точки не требуют аутентификации:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБЯЗАТЕЛЬНО: Установите базовый URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Выполняйте публичные вызовы API
    return 0;
}
```

### Using Moderation APIs (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБЯЗАТЕЛЬНО: Установите базовый URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Передайте SSO-токен модератора, чтобы аутентифицировать вызов
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

### Common Issues

1. **"URI must contain a hostname" error**: Убедитесь, что вы вызываете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` перед созданием ApiClient. Генератор cpp-restsdk не будет автоматически считывать URL сервера из спецификации OpenAPI.
2. **401 "missing-api-key" error**: Убедитесь, что вы вызываете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` перед созданием экземпляра DefaultAPI.
3. **Wrong API class**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модератора (аутентификация с помощью SSO-токена модератора).