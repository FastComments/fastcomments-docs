## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionResult200Response.php)

## Примјер

[inline-code-attrs-start title = 'createQuestionResult Примјер'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
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