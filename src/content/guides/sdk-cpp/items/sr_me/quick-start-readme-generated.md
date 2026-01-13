### Korišćenje autentifikovanih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (cpp-restsdk generator ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti vaš API ključ na ApiClient pre pravljenja autentifikovanih zahteva. Ako to ne uradite, zahtevi će propasti sa greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Postavite osnovni URL (izaberite svoju regiju)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ILI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBAVEZNO: Postavite vaš API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada izvršite autentifikovane API pozive
    return 0;
}
```

### Korišćenje javnih API-ja (PublicAPI)

Javne krajnje tačke ne zahtevaju autentifikaciju:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Izvršite javne API pozive
    return 0;
}
```

### Uobičajeni problemi

1. **Greška "URI must contain a hostname"**: Uverite se da pozovete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` pre nego što kreirate ApiClient. cpp-restsdk generator automatski ne čita URL servera iz OpenAPI specifikacije.
2. **Greška 401 "missing-api-key"**: Uverite se da pozovete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` pre nego što kreirate DefaultAPI instancu.
3. **Pogrešna API klasa**: Koristite `DefaultAPI` za autentifikovane zahteve sa serverske strane, `PublicAPI` za klijentske/javne zahteve.