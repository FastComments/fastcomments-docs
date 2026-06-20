### Використання автентифікованих API (DefaultAPI)

**Важливо:**
1. Ви повинні встановити базовий URL (генератор cpp-restsdk не читає його з OpenAPI spec)
2. Ви повинні встановити свій API ключ на ApiClient перед виконанням автентифікованих запитів. Якщо цього не зробите, запити завершаться помилкою 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБОВ'ЯЗКОВО: Встановіть базовий URL (оберіть свій регіон)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // АБО: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ОБОВ'ЯЗКОВО: Встановіть свій API-ключ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
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

    // ОБОВ'ЯЗКОВО: Встановіть базовий URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Виконайте публічні виклики API
    return 0;
}
```

### Використання ModerationApi (ModerationApi)

`ModerationApi` відповідає за панель модератора. Кожен метод приймає параметр `sso`, тому виклик виконується від імені модератора, автентифікованого через SSO (див. розділ SSO нижче про те, як створити токен):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ОБОВ'ЯЗКОВО: Встановіть базовий URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Передайте SSO-токен модератора для автентифікації виклику
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    auto response = moderationApi.getCount(
        boost::none,  // пошук за текстом
        boost::none,  // за IP з коментаря
        boost::none,  // фільтр
        boost::none,  // параметри пошуку
        boost::none,  // демо
        ssoToken      // sso
    ).get();

    return 0;
}
```

### Типові проблеми

1. **"URI must contain a hostname" error**: Переконайтеся, що ви викликаєте `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` перед створенням ApiClient. Генератор cpp-restsdk не читає автоматично URL сервера з OpenAPI spec.
2. **401 "missing-api-key" error**: Переконайтеся, що ви викликаєте `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` перед створенням екземпляра DefaultAPI.
3. **Wrong API class**: Використовуйте `DefaultApi` для автентифікованих запитів на стороні сервера, `PublicApi` для клієнтських/публічних запитів і `ModerationApi` для запитів до панелі модератора (автентифікація здійснюється за допомогою SSO-токена модератора).
---