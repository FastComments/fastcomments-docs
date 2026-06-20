### Korištenje autentifikovanih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (cpp-restsdk generator ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ na ApiClient prije slanja autentifikovanih zahtjeva. Ako to ne učinite, zahtjevi će završiti greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBAVEZNO: Postavite osnovni URL (odaberite svoju regiju)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBAVEZNO: Postavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada pravite autentifikovane API pozive
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

    // Pravite javne API pozive
    return 0;
}
```

### Korištenje Moderation API-ja (ModerationApi)

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

    // Proslijedite moderatorov SSO token da biste autentifikovali poziv
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

### Česti problemi

1. **"URI must contain a hostname" error**: Uvjerite se da pozivate `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prije kreiranja ApiClient-a. The cpp-restsdk generator ne čita automatski URL servera iz OpenAPI specifikacije.
2. **401 "missing-api-key" error**: Provjerite da pozovete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prije nego što kreirate DefaultAPI instancu.
3. **Wrong API class**: Koristite `DefaultApi` za zahtjeve koji se izvršavaju na serverskoj strani i zahtijevaju autentifikaciju, `PublicApi` za zahtjeve sa klijentske strane/javne zahtjeve, i `ModerationApi` za zahtjeve za moderatorsku nadzornu ploču (autentifikovane moderatorovim SSO tokenom).
---