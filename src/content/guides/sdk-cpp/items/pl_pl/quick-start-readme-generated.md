### Using Authenticated APIs (DefaultAPI)

**Ważne:**
1. Musisz ustawić podstawowy URL (generator cpp-restsdk nie odczytuje go z specyfikacji OpenAPI)
2. Musisz ustawić swój klucz API w ApiClient przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw podstawowy URL (wybierz swój region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // LUB: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // WYMAGANE: Ustaw swój klucz API
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Teraz wykonaj uwierzytelnione wywołania API
    return 0;
}
```

### Using Public APIs (PublicAPI)

Publiczne endpointy nie wymagają uwierzytelnienia:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw podstawowy URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Wykonaj publiczne wywołania API
    return 0;
}
```

### Using Moderation APIs (ModerationApi)

API `ModerationApi` napędza panel moderatora. Każda metoda przyjmuje parametr `sso`, więc wywołanie jest wykonywane w imieniu moderatora uwierzytelnionego przez SSO (zobacz sekcję SSO poniżej, aby dowiedzieć się, jak utworzyć token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw podstawowy URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Przekaż token SSO moderatora, aby uwierzytelnić wywołanie
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Common Issues

1. **Błąd "URI must contain a hostname"**: Upewnij się, że wywołujesz `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` przed utworzeniem ApiClient. Generator cpp-restsdk nie odczytuje automatycznie URL serwera z specyfikacji OpenAPI.
2. **Błąd 401 "missing-api-key"**: Upewnij się, że wywołujesz `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` przed utworzeniem instancji DefaultAPI.
3. **Nieprawidłowa klasa API**: Użyj `DefaultApi` dla żądań uwierzytelnionych po stronie serwera, `PublicApi` dla żądań po stronie klienta/publicznych, oraz `ModerationApi` dla żądań panelu moderatora (uwierzytelnionych tokenem SSO moderatora).