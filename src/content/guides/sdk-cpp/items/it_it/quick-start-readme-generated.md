### Uso delle API Autenticate (DefaultAPI)

**Importante:**
1. Devi impostare l'URL base (il generator cpp-restsdk non lo legge dallo spec OpenAPI)
2. Devi impostare la tua API key su ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBBLIGATORIO: Imposta l'URL base (scegli la tua regione)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OPPURE: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBBLIGATORIO: Imposta la tua API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Ora effettua chiamate API autenticate
    return 0;
}
```

### Uso delle API Pubbliche (PublicAPI)

Gli endpoint pubblici non richiedono autenticazione:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBBLIGATORIO: Imposta l'URL base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Effettua chiamate API pubbliche
    return 0;
}
```

### Uso delle API di Moderazione (ModerationApi)

La `ModerationApi` gestisce la dashboard dei moderatori. Ogni metodo accetta un parametro `sso` in modo che la chiamata venga eseguita per conto di un moderatore autenticato via SSO (vedi la sezione SSO sotto per come creare un token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBBLIGATORIO: Imposta l'URL base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Passa il token SSO del moderatore per autenticare la chiamata
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

### Problemi Comuni

1. **"URI must contain a hostname" error**: Assicurati di chiamare `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prima di creare l'ApiClient. Il generator cpp-restsdk non legge automaticamente l'URL del server dallo spec OpenAPI.
2. **401 "missing-api-key" error**: Assicurati di chiamare `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prima di creare l'istanza di DefaultAPI.
3. **Wrong API class**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste lato client/pubbliche, e `ModerationApi` per richieste della dashboard dei moderatori (autenticate con un token SSO del moderatore).