## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| questionId | string | query | Ne |  |
| questionIds | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odziv

Vrne: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultsResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodnjo vrstico za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$user_id = 'user_id_example'; // string
$start_date = 'start_date_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = 'question_ids_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getQuestionResults($tenant_id, $url_id, $user_id, $start_date, $question_id, $question_ids, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]