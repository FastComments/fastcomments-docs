### Utiliser les API authentifiées (DefaultAPI)

**Important :**
1. Vous devez définir l'URL de base (le générateur cpp-restsdk ne la lit pas depuis la spécification OpenAPI)
2. Vous devez définir votre clé API sur l'ApiClient avant d'effectuer des requêtes authentifiées. Sinon, les requêtes échoueront avec une erreur 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIS : Définir l'URL de base (choisissez votre région)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OU : config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // UE

    // REQUIS : Définir votre clé API
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Maintenant, effectuez des appels d'API authentifiés
    return 0;
}
```

### Utiliser les API publiques (PublicAPI)

Les endpoints publics ne nécessitent pas d'authentification :

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIS : Définir l'URL de base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Effectuez des appels API publics
    return 0;
}
```

### Problèmes courants

1. **Erreur « L'URI doit contenir un nom d'hôte »** : Assurez-vous d'appeler `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` avant de créer l'ApiClient. Le générateur cpp-restsdk ne lit pas automatiquement l'URL du serveur depuis la spécification OpenAPI.
2. **Erreur 401 « missing-api-key »** : Assurez-vous d'appeler `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` avant de créer l'instance DefaultAPI.
3. **Mauvaise classe API** : Utilisez `DefaultAPI` pour les requêtes authentifiées côté serveur, `PublicAPI` pour les requêtes côté client/publiques.