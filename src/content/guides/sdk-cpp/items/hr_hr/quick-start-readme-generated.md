### Korištenje autentificiranih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (cpp-restsdk generator ne čita ga iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ na ApiClient prije nego što izvršite autentificirane zahtjeve. Ako to ne učinite, zahtjevi će propasti s 401 greškom.

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

    // Sada izvršite autentificirane API pozive
    return 0;
}
```

### Korištenje javnih API-ja (PublicAPI)

Javni krajnji točke ne zahtijevaju autentifikaciju:

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

### Korištenje moderacijskih API-ja (ModerationApi)

`ModerationApi` pokreće moderacijski nadzorno ploču. Svaka metoda prihvaća `sso` parametar kako bi se poziv izvršio u ime moderatora autentificiranog putem SSO (pogledajte odjeljak SSO ispod za način stvaranja tokena):

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

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Česte poteškoće

1. **"URI mora sadržavati naziv hosta" greška**: Provjerite da pozovete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prije stvaranja ApiClient-a. cpp-restsdk generator ne čita automatski URL poslužitelja iz OpenAPI specifikacije.
2. **401 "missing-api-key" greška**: Provjerite da pozovete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prije stvaranja DefaultAPI instance.
3. **Pogrešna API klasa**: Koristite `DefaultApi` za autentificirane zahtjeve na strani poslužitelja, `PublicApi` za zahtjeve na klijentskoj strani/javni zahtjevi i `ModerationApi` za zahtjeve nadzorne ploče moderatora (autentificirano s SSO tokenom moderatora).