### Використання автентифікованих API (DefaultAPI)

**Важливо:**
1. Ви повинні встановити базовий URL (генератор cpp-restsdk не читає його зі специфікації OpenAPI)
2. Ви повинні встановити свій API-ключ на ApiClient перед виконанням автентифікованих запитів. Якщо ні, запити повернуть помилку 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБОВ'ЯЗКОВО: Вкажіть базовий URL (оберіть свій регіон)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // АБО: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ОБОВ'ЯЗКОВО: Вкажіть свій API-ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Тепер виконайте автентифіковані виклики API
    return 0;
}
```

### Використання публічних API (PublicAPI)

Публічні кінцеві точки не потребують автентифікації:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБОВ'ЯЗКОВО: Вкажіть базовий URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Виконайте відкриті виклики API
    return 0;
}
```

### Поширені проблеми

1. **"URI must contain a hostname" error**: Переконайтеся, що ви викликаєте `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` перед створенням ApiClient. Генератор cpp-restsdk не читає URL сервера зі специфікації OpenAPI автоматично.
2. **401 "missing-api-key" error**: Переконайтеся, що ви викликали `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` перед створенням екземпляра DefaultAPI.
3. **Wrong API class**: Використовуйте `DefaultAPI` для серверних автентифікованих запитів, `PublicAPI` для клієнтських/публічних запитів.