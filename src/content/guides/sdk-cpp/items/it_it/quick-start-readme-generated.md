### Utilizzo delle API Autenticate (DefaultAPI)

**Importante:**
1. Devi impostare l'URL di base (il generatore cpp-restsdk non lo legge dallo spec OpenAPI)
2. Devi impostare la tua chiave API sul ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

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

### Utilizzo delle API Pubbliche (PublicAPI)

Gli endpoint pubblici non richiedono autenticazione:

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

### Utilizzo delle API di Moderazione (ModerationApi)

La `ModerationApi` alimenta la dashboard del moderatore. Ogni metodo accetta un parametro `sso` così la chiamata viene eseguita per conto di un moderatore autenticato via SSO (vedi la sezione SSO più sotto per sapere come creare un token):

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

### Problemi comuni

1. **"URI must contain a hostname" error**: Assicurati di chiamare `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prima di creare l'ApiClient. Il generatore cpp-restsdk non legge automaticamente l'URL del server dallo spec OpenAPI.  
2. **401 "missing-api-key" error**: Assicurati di chiamare `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prima di creare l'istanza DefaultAPI.  
3. **Wrong API class**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste lato client/pubbliche, e `ModerationApi` per richieste della dashboard del moderatore (autenticate con un token SSO del moderatore).