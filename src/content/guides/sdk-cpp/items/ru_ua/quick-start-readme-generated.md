### Использование аутентифицированных API (DefaultAPI)

**Важно:**
1. Вы должны установить базовый URL (генератор cpp-restsdk не читает его из спецификации OpenAPI)
2. Вы должны установить ваш API-ключ в ApiClient перед выполнением аутентифицированных запросов. Если вы этого не сделаете, запросы завершатся ошибкой 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБЯЗАТЕЛЬНО: Установите базовый URL (выберите ваш регион)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ИЛИ: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ОБЯЗАТЕЛЬНО: Установите ваш API-ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Теперь можно выполнять аутентифицированные вызовы API
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

    // ОБЯЗАТЕЛЬНО: Установите базовый URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Выполните запросы к публичному API
    return 0;
}
```

### Распространенные проблемы

1. **"URI must contain a hostname" error**: Убедитесь, что вы вызываете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` перед созданием ApiClient. Генератор cpp-restsdk не считывает URL сервера автоматически из спецификации OpenAPI.
2. **401 "missing-api-key" error**: Убедитесь, что вы вызываете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` перед созданием экземпляра DefaultAPI.
3. **Wrong API class**: Используйте `DefaultAPI` для аутентифицированных запросов на стороне сервера, `PublicAPI` для клиентских/публичных запросов.