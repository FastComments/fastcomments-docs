### Authentifizierte APIs verwenden (DefaultAPI)

**Wichtig:**
1. Sie müssen die Basis-URL festlegen (cpp-restsdk-Generator liest sie nicht aus der OpenAPI-Spezifikation)
2. Sie müssen Ihren API-Schlüssel auf dem ApiClient setzen, bevor Sie authentifizierte Anfragen stellen. Wenn Sie das nicht tun, schlagen Anfragen mit einem 401-Fehler fehl.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Legen Sie die Basis-URL fest (wählen Sie Ihre Region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ERFORDERLICH: Legen Sie Ihren API-Schlüssel fest
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Jetzt authentifizierte API-Aufrufe durchführen
    return 0;
}
```

### Öffentliche APIs verwenden (PublicAPI)

Öffentliche Endpunkte erfordern keine Authentifizierung:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ERFORDERLICH: Legen Sie die Basis-URL fest
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Öffentliche API-Aufrufe durchführen
    return 0;
}
```

### Häufige Probleme

1. **"URI must contain a hostname" error**: Stellen Sie sicher, dass Sie `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` aufrufen, bevor Sie den ApiClient erstellen. Der cpp-restsdk-Generator liest die Server-URL nicht automatisch aus der OpenAPI-Spezifikation.
2. **401 "missing-api-key" error**: Stellen Sie sicher, dass Sie `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` aufrufen, bevor Sie die DefaultAPI-Instanz erstellen.
3. **Wrong API class**: Verwenden Sie `DefaultAPI` für serverseitige authentifizierte Anfragen, `PublicAPI` für clientseitige/öffentliche Anfragen.