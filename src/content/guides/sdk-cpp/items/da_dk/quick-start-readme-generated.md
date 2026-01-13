### Brug af godkendte API'er (DefaultAPI)

**Vigtigt:**
1. Du skal angive base-URL'en (cpp-restsdk generatoren læser den ikke fra OpenAPI-specifikationen)
2. Du skal angive din API-nøgle på ApiClient, før du foretager godkendte forespørgsler. Hvis du ikke gør det, vil forespørgsler fejle med en 401-fejl.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // PÅKRÆVET: Angiv base-URL'en (vælg din region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ELLER: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // PÅKRÆVET: Angiv din API-nøgle
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Foretag nu godkendte API-kald
    return 0;
}
```

### Brug af offentlige API'er (PublicAPI)

Offentlige endpoints kræver ikke godkendelse:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // PÅKRÆVET: Angiv base-URL'en
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Foretag offentlige API-kald
    return 0;
}
```

### Almindelige problemer

1. **"URI must contain a hostname" fejl**: Sørg for, at du kalder `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` før du opretter ApiClient. cpp-restsdk generatoren læser ikke automatisk server-URL'en fra OpenAPI-specifikationen.
2. **401 "missing-api-key" fejl**: Sørg for, at du kalder `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` før du opretter DefaultAPI-instansen.
3. **Forkert API-klasse**: Brug `DefaultAPI` til server-side godkendte forespørgsler, `PublicAPI` til klient-side/offentlige forespørgsler.