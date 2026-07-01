## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Réponse

Renvoie : [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionConfigsResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getQuestionConfigs'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configurer l'autorisation de la clé API : api_key
// Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si besoin
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // chaîne
$skip = 3.4; // flottant


try {
    $result = $apiInstance->getQuestionConfigs($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionConfigs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]