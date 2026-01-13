---
## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Retourne : [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackage200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple pour getTenantPackage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configurer l'autorisation de la clé API : api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getTenantPackage($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---