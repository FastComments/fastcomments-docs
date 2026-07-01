## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| questionId | string | query | Ne |  |
| questionIds | array | query | Ne |  |
| urlId | string | query | Ne |  |
| timeBucket | string | query | Ne |  |
| startDate | string | query | Ne |  |
| forceRecalculate | boolean | query | Ne |  |

## Odgovor

Vraća: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResultsResponse.php)

## Primjer

[inline-code-attrs-start title = 'aggregateQuestionResults Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfigurirajte autorizaciju API ključa: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, po potrebi
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadano.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'question_id' => 'question_id_example', // string
    'question_ids' => array('question_ids_example'), // string[]
    'url_id' => 'url_id_example', // string
    'time_bucket' => new \FastComments\Client\Model\\FastComments\Client\Model\AggregateTimeBucket(), // \FastComments\Client\Model\AggregateTimeBucket
    'start_date' => new \DateTime('2013-10-20T19:20:30+01:00'), // \DateTime
    'force_recalculate' => True, // bool
];


try {
    $result = $apiInstance->aggregateQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Izuzetak prilikom pozivanja DefaultApi->aggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]