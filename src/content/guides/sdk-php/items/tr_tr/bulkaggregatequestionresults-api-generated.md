## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| forceRecalculate | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkAggregateQuestionResultsResponse.php)

## Örnek

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// API anahtarı yetkilendirmesini yapılandırın: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdaki yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Özel bir HTTP istemcusu kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi iletin.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$bulk_aggregate_question_results_request = new \FastComments\Client\Model\BulkAggregateQuestionResultsRequest(); // \FastComments\Client\Model\BulkAggregateQuestionResultsRequest
$force_recalculate = True; // bool

try {
    $result = $apiInstance->bulkAggregateQuestionResults($tenant_id, $bulk_aggregate_question_results_request, $force_recalculate);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->bulkAggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]