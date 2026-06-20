### Korišćenje autentifikovanih API-ja (DefaultAPI)

**Važno:**
1. Morate podesiti base URL (generator cpp-restsdk ga ne čita iz OpenAPI specifikacije)
2. Morate podesiti svoj API key na ApiClient pre nego što napravite autentifikovane zahteve. Ako to ne uradite, zahtevi će se završiti greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Podesite base URL (izaberite region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ILI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBAVEZNO: Podesite svoj API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada pravite autentifikovane API pozive
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

    // OBAVEZNO: Podesite base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Pravimo javne API pozive
    return 0;
}
```

### Korišćenje moderacijskih API-ja (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Podesite base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Prosledite moderatorov SSO token da biste autentifikovali poziv
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

### Uobičajeni problemi

1. **"URI must contain a hostname" error**: Uverite se da pozivate `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` pre nego što kreirate ApiClient. Generator cpp-restsdk ne čita automatski server URL iz OpenAPI specifikacije.
2. **401 "missing-api-key" error**: Uverite se da pozivate `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` pre nego što kreirate instancu DefaultAPI.
3. **Wrong API class**: Koristite `DefaultApi` za server-side autentifikovane zahteve, `PublicApi` za klijent-side/javne zahteve, i `ModerationApi` za zahteve za moderator dashboard (autentifikovani moderatorovim SSO tokenom).
---