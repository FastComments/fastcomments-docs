### Gebruik van geauthentiseerde API's (DefaultAPI)

**Belangrijk:**
1. U moet de basis-URL instellen (de cpp-restsdk-generator leest deze niet uit de OpenAPI-specificatie)
2. U moet uw API-sleutel op de ApiClient instellen voordat u geauthentiseerde verzoeken doet. Als u dat niet doet, zullen verzoeken mislukken met een 401-fout.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL (choose your region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: Set your API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### Gebruik van openbare API's (PublicAPI)

Openbare eindpunten vereisen geen authenticatie:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Make public API calls
    return 0;
}
```

### Gebruik van moderatie-API's (ModerationApi)

De `ModerationApi` voedt het moderator-dashboard. Elke methode accepteert een `sso`-parameter zodat het aanroep wordt gedaan namens een SSO-geauthenticeerde moderator (zie de SSO-sectie hieronder voor hoe u een token maakt):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Pass the moderator's SSO token to authenticate the call
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Veelvoorkomende problemen

1. **"URI must contain a hostname"-fout**: Zorg ervoor dat u `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` aanroept vóór het maken van de ApiClient. De cpp-restsdk-generator leest de server-URL niet automatisch uit de OpenAPI-specificatie.  
2. **401 "missing-api-key"-fout**: Zorg ervoor dat u `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` aanroept vóór het maken van de DefaultAPI-instantie.  
3. **Verkeerde API-klasse**: Gebruik `DefaultApi` voor server-side geauthentificeerde verzoeken, `PublicApi` voor client-side/openbare verzoeken, en `ModerationApi` voor moderator-dashboard verzoeken (geauthentificeerd met een moderator SSO-token).