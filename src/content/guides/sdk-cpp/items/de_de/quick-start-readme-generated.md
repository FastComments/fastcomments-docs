### Verwendung authentifizierter APIs (DefaultAPI)

**Wichtig:**
1. Sie müssen die Basis-URL setzen (der cpp-restsdk-Generator liest sie nicht aus der OpenAPI-Spec)
2. Sie müssen Ihren API-Schlüssel am ApiClient setzen, bevor Sie authentifizierte Anfragen stellen. Andernfalls schlagen die Anfragen mit einem 401-Fehler fehl.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Basis-URL setzen (Region wählen)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // ODER: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ERFORDERLICH: API-Schlüssel setzen
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Jetzt authentifizierte API-Aufrufe tätigen
    return 0;
}
```

### Verwendung öffentlicher APIs (PublicAPI)

Öffentliche Endpunkte erfordern keine Authentifizierung:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Basis-URL setzen
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Öffentliche API-Aufrufe tätigen
    return 0;
}
```

### Verwendung der Moderations-APIs (ModerationApi)

Die `ModerationApi` betreibt das Moderator-Dashboard. Jede Methode akzeptiert einen `sso`-Parameter, sodass der Aufruf im Namen eines SSO-authentifizierten Moderators ausgeführt wird (siehe den SSO-Abschnitt unten
zum Erstellen eines Tokens):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Basis-URL setzen
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // SSO-Token des Moderators übergeben, um den Aufruf zu authentifizieren
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Häufige Probleme

1. **Fehler „URI must contain a hostname"**: Stellen Sie sicher, dass Sie `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` aufrufen, bevor Sie den ApiClient erstellen. Der cpp-restsdk-Generator liest die Server-URL nicht automatisch aus der OpenAPI-Spec.
2. **401-Fehler „missing-api-key"**: Stellen Sie sicher, dass Sie `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` aufrufen, bevor Sie die DefaultAPI-Instanz erstellen.
3. **Falsche API-Klasse**: Verwenden Sie `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen und `ModerationApi` für Moderator-Dashboard-Anfragen (authentifiziert mit einem Moderator-SSO-Token).