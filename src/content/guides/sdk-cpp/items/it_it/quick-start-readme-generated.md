### Using Authenticated APIs (DefaultAPI)

**Importante:**
1. Devi impostare l'URL base (il cpp-restsdk generator non lo legge dallo OpenAPI spec)
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
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBBLIGATORIO: Imposta la tua API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Ora esegui chiamate API autenticate
    return 0;
}
```

### Using Public APIs (PublicAPI)

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

    // Esegui chiamate API pubbliche
    return 0;
}
```

### Common Issues

1. **"URI must contain a hostname" error**: Assicurati di chiamare `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` prima di creare l'ApiClient. Il cpp-restsdk generator non legge automaticamente l'URL del server dallo OpenAPI spec.
2. **401 "missing-api-key" error**: Assicurati di chiamare `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` prima di creare l'istanza DefaultAPI.
3. **Wrong API class**: Usa `DefaultAPI` per richieste autenticate lato server, `PublicAPI` per richieste lato client/pubbliche.