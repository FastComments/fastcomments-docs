### Uporaba avtenticiranih API-jev (DefaultAPI)

**Pomembno:**
1. Nastaviti morate osnovni URL (generator cpp-restsdk ga ne bere iz OpenAPI specifikacije)
2. Pred izvajanjem avtenticiranih zahtev morate nastaviti vaš API ključ na ApiClient. Če tega ne storite, bodo zahteve zavrnjene z napako 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBVEZNO: Nastavite osnovni URL (izberite svojo regijo)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ALI: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBVEZNO: Nastavite svoj API ključ
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Zdaj izvedite avtenticirane klice API-ja
    return 0;
}
```

### Uporaba javnih API-jev (PublicAPI)

Javne API končne točke ne zahtevajo avtentikacije:

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

### Pogoste težave

1. **"URI must contain a hostname" error**: Prepričajte se, da kličete `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` pred ustvarjanjem ApiClient. Generator cpp-restsdk samodejno ne prebere URL strežnika iz OpenAPI specifikacije.
2. **401 "missing-api-key" error**: Prepričajte se, da kličete `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` pred ustvarjanjem instance DefaultAPI.
3. **Napačen razred API**: Uporabite `DefaultAPI` za strežniške avtenticirane zahteve, `PublicAPI` za odjemalske/javne zahteve.