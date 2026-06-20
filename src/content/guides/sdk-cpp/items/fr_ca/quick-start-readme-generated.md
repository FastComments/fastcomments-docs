### Utilisation des API authentifiées (DefaultAPI)

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

    // OBLIGATOIRE: Définir l'URL de base (choisissez votre région)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // États-Unis
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // UE

    // OBLIGATOIRE: Définir votre clé API
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Maintenant, effectuez des appels d'API authentifiés
    return 0;
}
```

### Utilisation des API publiques (PublicAPI)

Les points de terminaison publics ne nécessitent pas d'authentification :

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATOIRE: Définir l'URL de base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Effectuer des appels d'API publics
    return 0;
}
```

### Utilisation des API de modération (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBLIGATOIRE: Définir l'URL de base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Transmettez le jeton SSO du modérateur pour authentifier l'appel
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

### Problèmes courants

1. **Erreur "URI must contain a hostname"** : Assurez-vous d'appeler `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` avant de créer l'ApiClient. Le générateur cpp-restsdk ne lit pas automatiquement l'URL du serveur depuis la spécification OpenAPI.
2. **Erreur 401 "missing-api-key"** : Assurez-vous d'appeler `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` avant de créer l'instance DefaultAPI.
3. **Mauvaise classe d'API** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/publiques, et `ModerationApi` pour les requêtes du tableau de bord du modérateur (authentifiées avec un jeton SSO du modérateur).
---