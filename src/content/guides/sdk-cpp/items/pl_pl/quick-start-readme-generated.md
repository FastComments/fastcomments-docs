### Korzystanie z uwierzytelnionych API (DefaultAPI)

**Ważne:**
1. Musisz ustawić base URL (generator cpp-restsdk nie odczytuje go ze specyfikacji OpenAPI)
2. Musisz ustawić swój klucz API w ApiClient zanim wykonasz żądania uwierzytelnione. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw base URL (wybierz region)
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

Publiczne endpointy nie wymagają uwierzytelnienia:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // WYMAGANE: Ustaw base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Wykonaj publiczne wywołania API
    return 0;
}
```

### Typowe problemy

1. **Błąd "URI must contain a hostname"**: Upewnij się, że wywołujesz `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` przed utworzeniem ApiClient. Generator cpp-restsdk nie odczytuje automatycznie adresu serwera ze specyfikacji OpenAPI.
2. **Błąd 401 "missing-api-key"**: Upewnij się, że wywołujesz `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` przed utworzeniem instancji DefaultAPI.
3. **Nieprawidłowa klasa API**: Użyj `DefaultAPI` dla uwierzytelnionych żądań po stronie serwera, `PublicAPI` dla żądań po stronie klienta/publicznych.