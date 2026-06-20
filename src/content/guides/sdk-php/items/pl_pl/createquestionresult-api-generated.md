## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionResultResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład createQuestionResult'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację kluczem API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To opcjonalne, domyślnie użyty zostanie `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_question_result_body = new \FastComments\Client\Model\CreateQuestionResultBody(); // \FastComments\Client\Model\CreateQuestionResultBody

try {
    $result = $apiInstance->createQuestionResult($tenant_id, $create_question_result_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]