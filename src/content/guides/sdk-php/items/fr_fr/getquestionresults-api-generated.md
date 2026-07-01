## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Non |  |
| userId | string | query | Non |  |
| startDate | string | query | Non |  |
| questionId | string | query | Non |  |
| questionIds | string | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Retourne : [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultsResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurer l'autorisation de clé API : api_key
// Décommentez ci‑dessus pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // chaîne
$options = [
    'url_id' => 'url_id_example', // chaîne
    'user_id' => 'user_id_example', // chaîne
    'start_date' => 'start_date_example', // chaîne
    'question_id' => 'question_id_example', // chaîne
    'question_ids' => 'question_ids_example', // chaîne
    'skip' => 3.4, // flottant
];


try {
    $result = $apiInstance->getQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]