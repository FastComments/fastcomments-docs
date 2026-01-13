## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| questionId | string | query | Nie |  |
| questionIds | array | query | Nie |  |
| urlId | string | query | Nie |  |
| timeBucket | string | query | Nie |  |
| startDate | string | query | Nie |  |
| forceRecalculate | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResults200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład aggregateQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = array('question_ids_example'); // string[]
$url_id = 'url_id_example'; // string
$time_bucket = new \FastComments\Client\Model\\FastComments\Client\Model\AggregateTimeBucket(); // \FastComments\Client\Model\AggregateTimeBucket
$start_date = new \DateTime('2013-10-20T19:20:30+01:00'); // \DateTime
$force_recalculate = True; // bool

try {
    $result = $apiInstance->aggregateQuestionResults($tenant_id, $question_id, $question_ids, $url_id, $time_bucket, $start_date, $force_recalculate);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]