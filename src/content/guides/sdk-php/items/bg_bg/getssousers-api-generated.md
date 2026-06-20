## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | integer | query | Не |  |

## Отговор

Връща: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUsersResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getSSOUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на удостоверяване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате собствен HTTP клиент, подайте ваш клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е незадължително, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 56; // int

try {
    $result = $apiInstance->getSSOUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]