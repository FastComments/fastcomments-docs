### Korištenje autentificiranih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (generator cpp-restsdk ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ na ApiClient prije slanja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će završiti s 401 pogreškom.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL (choose your region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: Set your API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### Korištenje javnih API-ja (PublicAPI)

Javni endpointi ne zahtijevaju autentifikaciju:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Make public API calls
    return 0;
}
```

### Uobičajeni problemi

1. **Greška "URI must contain a hostname"**: Provjerite da pozivate `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prije stvaranja ApiClient-a. Generator cpp-restsdk ne učitava automatski URL servera iz OpenAPI specifikacije.
2. **401 greška "missing-api-key"**: Provjerite da pozivate `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prije stvaranja DefaultAPI instance.
3. **Pogrešna API klasa**: Koristite `DefaultAPI` za server-side autentificirane zahtjeve, `PublicAPI` za zahtjeve na strani klijenta/javne zahtjeve.