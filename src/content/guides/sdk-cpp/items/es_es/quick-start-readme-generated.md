### Uso de APIs autenticadas (DefaultAPI)

**Importante:**
1. Debes establecer la URL base (el generador cpp-restsdk no la lee desde la OpenAPI spec)
2. Debes establecer tu clave de API en el ApiClient antes de hacer solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUERIDO: Establece la URL base (elige tu región)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // O: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUERIDO: Establece tu clave de API
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Ahora realiza llamadas a la API autenticadas
    return 0;
}
```

### Uso de APIs públicas (PublicAPI)

Los endpoints públicos no requieren autenticación:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUERIDO: Establece la URL base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Realiza llamadas a la API públicas
    return 0;
}
```

### Problemas comunes

1. **"URI must contain a hostname" error**: Asegúrate de llamar a `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` antes de crear el ApiClient. El cpp-restsdk generator no lee automáticamente la URL del servidor desde la OpenAPI spec.
2. **401 "missing-api-key" error**: Asegúrate de llamar a `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` antes de crear la instancia de DefaultAPI.
3. **Wrong API class**: Usa `DefaultAPI` para solicitudes autenticadas del lado del servidor, y `PublicAPI` para solicitudes del lado del cliente/públicas.