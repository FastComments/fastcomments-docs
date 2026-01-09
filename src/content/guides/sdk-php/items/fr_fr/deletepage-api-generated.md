## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Réponse

Renvoie: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeletePageAPIResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de deletePage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous voulez utiliser un client HTTP personnalisé, fournissez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // C'est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->deletePage($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deletePage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]