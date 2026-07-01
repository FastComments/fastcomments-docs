## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Réponse

Renvoie : [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResultsResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple aggregateQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configurer l'autorisation de la clé API : api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Décommentez ci‑dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // chaîne
$options = [
    'question_id' => 'question_id_example', // chaîne
    'question_ids' => array('question_ids_example'), // string[]
    'url_id' => 'url_id_example', // chaîne
    'time_bucket' => new \FastComments\Client\Model\\FastComments\Client\Model\AggregateTimeBucket(), // \FastComments\Client\Model\AggregateTimeBucket
    'start_date' => new \DateTime('2013-10-20T19:20:30+01:00'), // \DateTime
    'force_recalculate' => True, // bool
];


try {
    $result = $apiInstance->aggregateQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]