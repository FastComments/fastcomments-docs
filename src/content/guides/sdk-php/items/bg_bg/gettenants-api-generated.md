## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| meta | string | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getTenants'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на удостоверяване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу, за да зададете префикс (e.g. Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате собствен HTTP клиент, предайте своя клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$meta = 'meta_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenants($tenant_id, $meta, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenants: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]