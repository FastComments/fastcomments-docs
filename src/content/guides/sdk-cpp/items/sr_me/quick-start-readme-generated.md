### Korišćenje autentifikovanih API‑ja (DefaultAPI)

**Važno:**
1. Morate postaviti osnovni URL (generator cpp‑restsdk ga ne čita iz OpenAPI specifikacije)
2. Morate postaviti svoj API ključ u ApiClient prije nego što napravite autentifikovane zahtjeve. Ako to ne uradite, zahtjevi će propasti s greškom 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATORNO: Postavite osnovni URL (odaberite svoju regiju)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // SAD
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBLIGATORNO: Postavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Sada napravite autentifikovane API pozive
    return 0;
}
```

### Korišćenje javnih API‑ja (PublicAPI)

Javni endpointi ne zahtijevaju autentifikaciju:

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

    // Napravite javne API pozive
    return 0;
}
```

### Korišćenje moderacijskih API‑ja (ModerationApi)

`ModerationApi` pokreće moderatorsku kontrolnu ploču. Svaka metoda prihvata `sso` parametar tako da se poziv izvršava u ime SSO‑autentifikovanog moderatora (pogledajte odeljak SSO ispod za način kreiranja tokena):

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

    // Proslijedite SSO token moderatora da autentifikujete poziv
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Česti problemi

1. **Greška „URI must contain a hostname“**: Uvjerite se da ste pozvali `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prije kreiranja ApiClient‑a. Generator cpp‑restsdk automatski ne čita URL servera iz OpenAPI specifikacije.
2. **Greška 401 „missing-api-key“**: Uvjerite se da ste pozvali `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prije kreiranja instance DefaultAPI.
3. **Pogrešna API klasa**: Koristite `DefaultApi` za server‑side autentifikovane zahtjeve, `PublicApi` za klijentske/javne zahtjeve i `ModerationApi` za zahtjeve moderatorske kontrolne ploče (autentifikovane SSO tokenom moderatora).