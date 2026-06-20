### Verwendung authentifizierter APIs (DefaultAPI)

**Wichtig:**
1. Sie müssen die Basis-URL festlegen (der cpp-restsdk-Generator liest sie nicht aus der OpenAPI-Spezifikation)
2. Sie müssen Ihren API-Schlüssel im ApiClient setzen, bevor Sie authentifizierte Anfragen stellen. Wenn Sie dies nicht tun, schlagen die Anfragen mit einem 401-Fehler fehl.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Basis-URL festlegen (wählen Sie Ihre Region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ERFORDERLICH: Ihren API-Schlüssel setzen
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Jetzt authentifizierte API-Aufrufe durchführen
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

    // ERFORDERLICH: Basis-URL festlegen
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Öffentliche API-Aufrufe durchführen
    return 0;
}
```

### Verwendung der Moderations-APIs (ModerationApi)

Die `ModerationApi` ist für das Moderator-Dashboard zuständig. Jede Methode akzeptiert einen `sso`-Parameter, sodass der Aufruf im Auftrag eines per SSO authentifizierten Moderators ausgeführt wird (siehe den SSO-Abschnitt unten, um zu erfahren, wie man ein Token erstellt):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Basis-URL festlegen
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Übergeben Sie den SSO-Token des Moderators, um den Aufruf zu authentifizieren
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

### Häufige Probleme

1. **"URI must contain a hostname" error**: Stellen Sie sicher, dass Sie `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` aufrufen, bevor Sie den ApiClient erstellen. Der cpp-restsdk-Generator liest die Server-URL nicht automatisch aus der OpenAPI-Spezifikation.
2. **401 "missing-api-key" error**: Stellen Sie sicher, dass Sie `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` aufrufen, bevor Sie die DefaultAPI-Instanz erstellen.
3. **Falsche API-Klasse**: Verwenden Sie `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen und `ModerationApi` für Anfragen des Moderator-Dashboards (authentifiziert mit einem Moderator-SSO-Token).
---