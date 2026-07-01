### Brug af autentificerede API'er (DefaultAPI)

**Vigtigt:**
1. Du skal angive base‑URL’en (cpp‑restsdk‑generatoren læser den ikke fra OpenAPI‑specifikationen)
2. Du skal indstille din API‑nøgle på ApiClient, før du foretager autentificerede anmodninger. Hvis du ikke gør det, vil anmodninger fejle med en 401‑fejl.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // KRÆVET: Indstil basis-URL'en (vælg din region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ELLER: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // KRÆVET: Indstil din API-nøgle
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Foretag nu godkendte API‑kald
    return 0;
}
```

### Brug af offentlige API'er (PublicAPI)

Offentlige slutpunkter kræver ikke godkendelse:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // KRÆVET: Indstil basis-URL'en
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Foretag offentlige API‑kald
    return 0;
}
```

### Brug af moderations‑API'er (ModerationApi)

`ModerationApi` driver moderator‑dashboardet. Hver metode accepterer en `sso`‑parameter, så kaldet udføres på vegne af en SSO‑godkendt moderator (se SSO‑sektionen nedenfor for, hvordan du opretter et token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // KRÆVET: Indstil basis-URL'en
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Videregiv moderatorens SSO-token for at autentificere kaldet
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Almindelige problemer

1. **"URI skal indeholde et hostnavn"-fejl**: Sørg for at kalde `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` før du opretter ApiClient. cpp‑restsdk‑generatoren læser ikke automatisk server‑URL’en fra OpenAPI‑specifikationen.
2. **401 "missing-api-key"-fejl**: Sørg for at kalde `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` før du opretter DefaultAPI‑instansen.
3. **Forkert API‑klasse**: Brug `DefaultApi` til server‑side autentificerede anmodninger, `PublicApi` til klient‑side/offentlige anmodninger, og `ModerationApi` til moderator‑dashboard‑anmodninger (godkendt med et moderator‑SSO‑token).