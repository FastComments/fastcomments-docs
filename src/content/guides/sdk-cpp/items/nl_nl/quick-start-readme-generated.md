### Gebruik van geauthenticeerde API's (DefaultAPI)

**Belangrijk:**
1. U moet de basis-URL instellen (de cpp-restsdk-generator leest deze niet uit de OpenAPI-specificatie)
2. U moet uw API-sleutel instellen op de ApiClient voordat u geauthenticeerde verzoeken doet. Als u dit niet doet, zullen verzoeken mislukken met een 401-fout.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // VEREIST: Stel de basis-URL in (kies uw regio)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OF: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // VEREIST: Stel uw API-sleutel in
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Maak nu geauthenticeerde API-aanroepen
    return 0;
}
```

### Gebruik van openbare API's (PublicAPI)

Openbare endpoints vereisen geen authenticatie:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // VEREIST: Stel de basis-URL in
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Maak openbare API-aanroepen
    return 0;
}
```

### Veelvoorkomende problemen

1. **"URI must contain a hostname" error**: Zorg ervoor dat u `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` aanroept voordat u de ApiClient maakt. De cpp-restsdk-generator leest de server-URL niet automatisch uit de OpenAPI-specificatie.
2. **401 "missing-api-key" error**: Zorg ervoor dat u `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` aanroept voordat u een DefaultAPI-instantie maakt.
3. **Wrong API class**: Gebruik `DefaultAPI` voor server-side geauthenticeerde verzoeken, `PublicAPI` voor client-side/openbare verzoeken.