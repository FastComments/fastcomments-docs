## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | у запиті | Так |  |

## Відповідь

Повертає: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад createTenant'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштувати авторизацію за ключем API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це опціонально, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // рядок
$create_tenant_body = new \FastComments\Client\Model\CreateTenantBody(); // \FastComments\Client\Model\CreateTenantBody

try {
    $result = $apiInstance->createTenant($tenant_id, $create_tenant_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenant: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]