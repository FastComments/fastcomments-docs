### Korišćenje autentifikovanih API‑ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (cpp-restsdk generator ne čita URL iz OpenAPI specifikacije)
2. Morate postaviti vaš API ključ na ApiClient pre slanja autentifikovanih zahteva. Ako to ne uradite, zahtevi će propasti sa greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATORNO: Postavite osnovni URL (izaberite svoju regiju)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ILI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBLIGATORNO: Postavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada izvršite autentifikovane API pozive
    return 0;
}
```

### Korišćenje javnih API‑ja (PublicAPI)

Javni endpointi ne zahtevaju autentifikaciju:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATORNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Izvršite javne API pozive
    return 0;
}
```

### Korišćenje API‑ja za moderaciju (ModerationApi)

`ModerationApi` omogućava rad na moderator tabli. Svaka metoda prihvata `sso` parametar, pa se poziv izvršava u ime SSO‑autentifikovanog moderatora (pogledajte odeljak o SSO‑u ispod za način kreiranja tokena):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATORNO: Postavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Prosledite SSO token moderatora kako biste autentifikovali poziv
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Uobičajeni problemi

1. **Greška „URI must contain a hostname“**: Uverite se da pozovete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` pre kreiranja `ApiClient`‑a. cpp-restsdk generator ne čita automatski URL servera iz OpenAPI specifikacije.
2. **401 greška „missing-api-key“**: Uverite se da pozovete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` pre kreiranja instance `DefaultAPI`.
3. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve i `ModerationApi` za zahteve moderator table (autentifikovani SSO tokenom moderatora).