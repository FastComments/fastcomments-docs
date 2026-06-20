### Korzystanie z uwierzytelnionych API (DefaultAPI)

**Ważne:**
1. Musisz ustawić adres bazowy (generator cpp-restsdk nie pobiera go ze specyfikacji OpenAPI)
2. Musisz ustawić swój klucz API w ApiClient przed wykonywaniem żądań uwierzytelnionych. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw adres bazowy (wybierz region)
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

### Korzystanie z publicznych API (PublicAPI)

Punkty końcowe publiczne nie wymagają uwierzytelnienia:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw adres bazowy
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Wykonaj publiczne wywołania API
    return 0;
}
```

### Korzystanie z Moderation APIs (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw adres bazowy
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Przekaż token SSO moderatora, aby uwierzytelnić wywołanie
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

### Częste problemy

1. **Błąd „URI musi zawierać nazwę hosta”**: Upewnij się, że wywołujesz `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` przed utworzeniem ApiClient. Generator cpp-restsdk nie odczytuje automatycznie adresu serwera ze specyfikacji OpenAPI.
2. **Błąd 401 "missing-api-key"**: Upewnij się, że wywołujesz `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` przed utworzeniem instancji DefaultAPI.
3. **Nieprawidłowa klasa API**: Użyj `DefaultApi` dla żądań uwierzytelnionych po stronie serwera, `PublicApi` dla żądań po stronie klienta/publicznych, oraz `ModerationApi` dla żądań panelu moderatora (uwierzytelnianych tokenem SSO moderatora).