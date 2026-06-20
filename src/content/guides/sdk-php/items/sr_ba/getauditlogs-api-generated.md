## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |
| order | string | query | Не |  |
| after | number | query | Не |  |
| before | number | query | Не |  |

## Одговор

Враћа: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogsResponse.php)

## Пример

[inline-code-attrs-start title = 'getAuditLogs Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигуришите ауторизацију API кључа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако желите користити прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float
$order = new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(); // \FastComments\Client\Model\SORTDIR
$after = 3.4; // float
$before = 3.4; // float

try {
    $result = $apiInstance->getAuditLogs($tenant_id, $limit, $skip, $order, $after, $before);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]