### Uporaba avtenticiranih API-jev (DefaultAPI)

**Pomembno:**
1. Nastaviti morate osnovni URL (cpp-restsdk generator ga ne bere iz OpenAPI specifikacije)
2. Preden izvedete avtenticirane zahteve, morate nastaviti svoj API ključ na ApiClient. Če tega ne storite, bodo zahteve neuspešne z 401 napako.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavite osnovni URL (izberite regijo)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ALI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBVEZNO: Nastavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Zdaj naredite avtenticirane klice API-ja
    return 0;
}
```

### Uporaba javnih API-jev (PublicAPI)

Javne končne točke ne zahtevajo avtentikacije:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Izvedite javne klice API-ja
    return 0;
}
```

### Uporaba moderacijskih API-jev (ModerationApi)

`ModerationApi` poganja nadzorno ploščo moderatorja. Vsaka metoda sprejme parameter `sso`, tako da se klic izvede v imenu moderatorja, avtenticiranega z SSO (glejte razdelek SSO spodaj za navodila, kako ustvariti žeton):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavite osnovni URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Posredujte moderatorjev SSO žeton za avtentikacijo klica
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

### Pogoste težave

1. **"URI must contain a hostname" error**: Poskrbite, da pokličete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` preden ustvarite ApiClient. cpp-restsdk generator samodejno ne prebere URL-ja strežnika iz OpenAPI specifikacije.
2. **401 "missing-api-key" error**: Poskrbite, da pokličete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` preden ustvarite instanco DefaultAPI.
3. **Wrong API class**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za zahteve na strani odjemalca/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja (avtenticirano z moderatorjevim SSO žetonom).