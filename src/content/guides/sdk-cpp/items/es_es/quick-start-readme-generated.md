### Usando APIs Autenticadas (DefaultAPI)

**Importante:**
1. Debes establecer la URL base (el generador cpp-restsdk no la lee del spec OpenAPI)
2. Debes establecer tu clave API en el ApiClient antes de hacer solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUERIDO: Establecer la URL base (elige tu región)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // O: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUERIDO: Establecer tu clave API
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Ahora realiza llamadas API autenticadas
    return 0;
}
```

### Usando APIs Públicas (PublicAPI)

Los endpoints públicos no requieren autenticación:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUERIDO: Establecer la URL base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Realizar llamadas API públicas
    return 0;
}
```

### Usando APIs de Moderación (ModerationApi)

El `ModerationApi` alimenta el panel de moderador. Cada método acepta un parámetro `sso` para que la llamada se ejecute en nombre de un moderador autenticado vía SSO (consulta la sección SSO a continuación para saber cómo crear un token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUERIDO: Establecer la URL base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Pasar el token SSO del moderador para autenticar la llamada
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Problemas Comunes

1. **"URI must contain a hostname" error**: Asegúrate de llamar a `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` antes de crear el ApiClient. El generador cpp-restsdk no lee automáticamente la URL del servidor del spec OpenAPI.
2. **401 "missing-api-key" error**: Asegúrate de llamar a `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` antes de crear la instancia DefaultAPI.
3. **Wrong API class**: Usa `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado cliente/públicas, y `ModerationApi` para solicitudes del panel de moderador (autenticadas con un token SSO de moderador).