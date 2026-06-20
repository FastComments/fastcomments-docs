### Korištenje autentificiranih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (generator cpp-restsdk ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ na ApiClient prije slanja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će završiti s greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Postavite osnovni URL (odaberite svoju regiju)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // SAD
    // ILI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBAVEZNO: Postavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada napravite autentificirane API pozive
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

    // OBAVEZNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Napravite javne API pozive
    return 0;
}
```

### Korištenje moderacijskih API-ja (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Proslijedite SSO token moderatora za autentifikaciju poziva
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

1. **"URI must contain a hostname" greška**: Provjerite da pozivate `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prije kreiranja ApiClienta. Generator cpp-restsdk automatski ne čita URL servera iz OpenAPI specifikacije.
2. **401 "missing-api-key" greška**: Provjerite da pozivate `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prije kreiranja instance DefaultAPI.
3. **Pogrešna API klasa**: Koristite `DefaultApi` za autentificirane zahtjeve na strani poslužitelja, `PublicApi` za zahtjeve s klijentske strane/javne zahtjeve, i `ModerationApi` za zahtjeve nadzorne ploče moderatora (autentificirane SSO tokenom moderatora).