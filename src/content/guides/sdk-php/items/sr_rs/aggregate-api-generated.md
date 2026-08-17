Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Не |  |
| includeStats | boolean | query | Не |  |

## Response

Враћа: [`AggregateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateResponse.php)

## Пример

[inline-code-attrs-start title = 'пример агрегирања'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигуриши ауторизацију API кључа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Одкоментариши испод да подесиш префикс (нпр. Bearer) за API кључ, ако је потребно
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако желиш да користиш прилагођени http клијент, проследи свом клијенту који имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевано.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$aggregation_request = new \FastComments\Client\Model\AggregationRequest(); // \FastComments\Client\Model\AggregationRequest
$options = [
    'parent_tenant_id' => 'parent_tenant_id_example', // string
    'include_stats' => True, // bool
];


try {
    $result = $apiInstance->aggregate($tenant_id, $aggregation_request, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]