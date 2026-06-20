## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Renvoie : [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionResult'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configuration de l'authentification par clé API : api_key
// Décommentez ci-dessous pour configurer un préfixe (par ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // chaîne
$id = 'id_example'; // chaîne

try {
    $result = $apiInstance->getQuestionResult($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]