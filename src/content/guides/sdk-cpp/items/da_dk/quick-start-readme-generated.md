### Brug af autentificerede API'er (DefaultAPI)

**Vigtigt:**
1. Du skal indstille base-URL'en (cpp-restsdk generator læser den ikke fra OpenAPI spec)
2. Du skal indstille din API-nøgle på ApiClient, før du foretager autentificerede forespørgsler. Hvis du ikke gør det, vil forespørgsler fejle med en 401-fejl.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // PÅKRÆVET: Indstil base-URL'en (vælg din region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ELLER: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // PÅKRÆVET: Indstil din API-nøgle
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Foretag nu autentificerede API-kald
    return 0;
}
```

### Brug af offentlige API'er (PublicAPI)

Offentlige endepunkter kræver ikke godkendelse:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // PÅKRÆVET: Indstil base-URL'en
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Foretag offentlige API-kald
    return 0;
}
```

### Brug af Moderation-API'er (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // PÅKRÆVET: Indstil base-URL'en
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Send moderatorens SSO-token for at autentificere kaldet
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

### Almindelige problemer

1. **"URI must contain a hostname" error**: Sørg for, at du kalder `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` før du opretter ApiClient. cpp-restsdk generator læser ikke automatisk server-URL'en fra OpenAPI spec.
2. **401 "missing-api-key" error**: Sørg for, at du kalder `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` før du opretter en DefaultAPI-instans.
3. **Wrong API class**: Brug `DefaultApi` til serverside-autentificerede forespørgsler, `PublicApi` til klientside/offentlige forespørgsler, og `ModerationApi` til moderator-dashboard-forespørgsler (autentificeret med et moderator-SSO-token).