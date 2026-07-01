### Utilisation des API authentifiées (DefaultAPI)

**Important :**
1. Vous devez définir l'URL de base (le générateur cpp-restsdk ne la lit pas à partir du spec OpenAPI)
2. Vous devez définir votre clé API sur l'ApiClient avant d'effectuer des requêtes authentifiées. Sinon, les requêtes échoueront avec une erreur 401.

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

### Utilisation des API publiques (PublicAPI)

Les points de terminaison publics ne nécessitent pas d'authentification :

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

### Utilisation des API de modération (ModerationApi)

L'`ModerationApi` alimente le tableau de bord du modérateur. Chaque méthode accepte un paramètre `sso` afin que l'appel s'exécute au nom d'un modérateur authentifié via SSO (voir la section SSO ci‑dessous pour savoir comment créer un jeton) :

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

### Problèmes courants

1. **Erreur "URI must contain a hostname"** : Assurez‑vous d'appeler `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` avant de créer l'ApiClient. Le générateur cpp-restsdk ne lit pas automatiquement l'URL du serveur depuis le spec OpenAPI.
2. **Erreur 401 "missing-api-key"** : Assurez‑vous d'appeler `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` avant de créer l'instance DefaultAPI.
3. **Mauvaise classe API** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/public, et `ModerationApi` pour les requêtes du tableau de bord du modérateur (authentifiées avec un jeton SSO de modérateur).