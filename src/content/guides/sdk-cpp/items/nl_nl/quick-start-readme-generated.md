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

### Gebruik van publieke API's (PublicAPI)

Publieke endpoints vereisen geen authenticatie:

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

### Gebruik van moderatie-API's (ModerationApi)

De `ModerationApi` drijft het moderator-dashboard aan. Elke methode accepteert een `sso`-parameter zodat de oproep namens een via SSO geauthenticeerde moderator wordt uitgevoerd (zie de SSO-sectie hieronder voor hoe je een token maakt):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // VEREIST: Stel de basis-URL in
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Geef de SSO-token van de moderator mee om de oproep te authenticeren
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

### Veelvoorkomende problemen

1. **fout "URI moet een hostnaam bevatten"**: Zorg ervoor dat u `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` aanroept voordat u de ApiClient aanmaakt. De cpp-restsdk-generator leest de server-URL niet automatisch uit de OpenAPI-specificatie.
2. **401 "missing-api-key" fout**: Zorg ervoor dat u `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` aanroept voordat u de DefaultAPI-instantie maakt.
3. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server-side geauthenticeerde verzoeken, `PublicApi` voor client-side/publieke verzoeken en `ModerationApi` voor moderator-dashboardverzoeken (geauthenticeerd met een moderator SSO-token).