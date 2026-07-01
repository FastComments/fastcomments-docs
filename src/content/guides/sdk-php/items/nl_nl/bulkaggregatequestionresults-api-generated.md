## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| forceRecalculate | boolean | query | Nee |  |

## Response

Retourneert: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkAggregateQuestionResultsResponse.php)

## Example

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutelauthorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als je een aangepaste http-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
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