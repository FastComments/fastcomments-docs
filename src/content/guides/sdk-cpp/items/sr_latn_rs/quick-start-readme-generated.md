### Korišćenje autentifikovanih API-ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (cpp-restsdk generator ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ na ApiClient pre nego što napravite autentifikovane zahteve. Ako to ne uradite, zahtevi će propasti sa greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATNO: Postavite osnovni URL (odaberite vašu regiju)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // SAD
    // ILI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBLIGATNO: Postavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada napravite autentifikovane API pozive
    return 0;
}
```

### Korišćenje javnih API-ja (PublicAPI)

Javni krajnji ostvarivači ne zahtevaju autentifikaciju:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Napravite javne API pozive
    return 0;
}
```

### Korišćenje Moderacionih API-ja (ModerationApi)

`ModerationApi` pokreće moderatorsku kontrolnu tablu. Svaki metod prihvata `sso` parametar tako da se poziv izvršava u ime moderatora autentifikovanog putem SSO (pogledajte SSO sekciju ispod za način kreiranja tokena):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Prosledite SSO token moderatora da autentifikujete poziv
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Uobičajeni problemi

1. **Greška "URI must contain a hostname"**: Proverite da pozovete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` pre kreiranja ApiClient‑a. cpp-restsdk generator ne čita automatski URL servera iz OpenAPI specifikacije.
2. **Greška 401 "missing-api-key"**: Proverite da pozovete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` pre kreiranja DefaultAPI instance.
3. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve, i `ModerationApi` za zahteve moderatorske kontrolne table (autentifikovane SSO tokenom moderatora).