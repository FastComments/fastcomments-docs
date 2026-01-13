### Korišćenje autentifikovanih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (cpp-restsdk generator ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ na ApiClient pre slanja autentifikovanih zahteva. Ako to ne uradite, zahtevi će biti odbijeni sa 401 greškom.

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

### Korišćenje javnih API-ja (PublicAPI)

Javni endpointi ne zahtevaju autentifikaciju:

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

1. **"URI must contain a hostname" error**: Uverite se da pozovete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` pre nego što kreirate ApiClient. cpp-restsdk generator ne čita automatski URL servera iz OpenAPI specifikacije.
2. **401 "missing-api-key" error**: Uverite se da pozovete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` pre nego što kreirate instancu DefaultAPI.
3. **Pogrešna API klasa**: Koristite `DefaultAPI` za autentifikovane zahteve na serverskoj strani, a `PublicAPI` za klijentske/javne zahteve.