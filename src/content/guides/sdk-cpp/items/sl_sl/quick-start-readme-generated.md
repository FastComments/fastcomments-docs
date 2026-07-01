### Uporaba avtenticiranih API-jev (DefaultAPI)

**Pomembno:**
1. Morate nastaviti osnovni URL (cpp‑restsdk generator ga ne prebere iz OpenAPI specifikacije)
2. Morate nastaviti svoj API ključ v ApiClient pred izvedbo avtenticiranih zahtev. Če tega ne storite, bodo zahteve neuspešne z napako 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavi osnovni URL (izberi svojo regijo)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ALI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBVEZNO: Nastavi svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Zdaj izvedi avtenticirane klice API
    return 0;
}
```

### Uporaba javnih API-jev (PublicAPI)

Javni končni naslovi ne zahtevajo avtentikacije:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavi osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Izvedi javne klice API
    return 0;
}
```

### Uporaba moderacijskih API-jev (ModerationApi)

`ModerationApi` poganja nadzorno ploščo moderatorjev. Vsaka metoda sprejme parameter `sso`, tako da se klic izvede v imenu moderatorja, avtenticiranega prek SSO (glejte sekcijo SSO spodaj za ustvarjanje žetona):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavi osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Posreduj SSO žeton moderatorja, da overiš klic
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Pogoste težave

1. **Napaka “URI must contain a hostname”**: Prepričajte se, da pred ustvarjanjem `ApiClient` pokličete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`. Generator cpp‑restsdk ne prebere samodejno URL‑ja strežnika iz OpenAPI specifikacije.
2. **Napaka 401 “missing-api-key”**: Prepričajte se, da pred ustvarjanjem instance `DefaultAPI` pokličete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`.
3. **Napačen API razred**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja (avtenticirane s SSO žetonom moderatorja).