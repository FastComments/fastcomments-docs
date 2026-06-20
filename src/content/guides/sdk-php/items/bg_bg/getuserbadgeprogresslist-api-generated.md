## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgeProgressListResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadgeProgressList'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигуриране на удостоверяване с API ключ: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Премахнете коментара по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако искате да използвате персонализиран HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Това е по избор, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadgeProgressList($tenant_id, $user_id, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressList: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]