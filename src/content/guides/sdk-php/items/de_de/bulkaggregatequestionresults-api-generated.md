## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| forceRecalculate | boolean | query | No |  |

## Antwort

Gibt zurück: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkAggregateQuestionResultsResponse.php)

## Beispiel

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen eigenen HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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