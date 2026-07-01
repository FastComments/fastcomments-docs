### Использование аутентифицированных API (DefaultAPI)

**Важно:**
1. Вы должны установить базовый URL (генератор cpp-restsdk не читает его из спецификации OpenAPI)
2. Вы должны установить ваш API‑ключ в ApiClient перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершатся ошибкой 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Установите базовый URL (выберите ваш регион)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: Установите ваш API‑ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Теперь делайте аутентифицированные вызовы API
    return 0;
}
```

### Использование публичных API (PublicAPI)

Публичные конечные точки не требуют аутентификации:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Установите базовый URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Выполняйте публичные API вызовы
    return 0;
}
```

### Использование API модерации (ModerationApi)

`ModerationApi` обслуживает панель модератора. Каждый метод принимает параметр `sso`, поэтому вызов выполняется от имени модератора, аутентифицированного через SSO (см. раздел SSO ниже, чтобы узнать, как создать токен):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Установите базовый URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Передайте SSO‑токен модератора для аутентификации вызова
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Общие проблемы

1. **Ошибка "URI must contain a hostname"**: Убедитесь, что вы вызываете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` перед созданием ApiClient. Генератор cpp-restsdk не читает автоматически URL сервера из спецификации OpenAPI.
2. **Ошибка 401 "missing-api-key"**: Убедитесь, что вы вызываете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` перед созданием экземпляра DefaultAPI.
3. **Неправильный класс API**: Используйте `DefaultApi` для серверных аутентифицированных запросов, `PublicApi` для клиентских/публичных запросов и `ModerationApi` для запросов панели модератора (аутентифицированных с помощью SSO‑токена модератора).