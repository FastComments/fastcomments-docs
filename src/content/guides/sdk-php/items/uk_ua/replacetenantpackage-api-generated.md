## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад replaceTenantPackage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації API ключем: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб задати префікс (наприклад, Bearer) для API ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // рядок
$id = 'id_example'; // рядок
$replace_tenant_package_body = new \FastComments\Client\Model\ReplaceTenantPackageBody(); // \FastComments\Client\Model\ReplaceTenantPackageBody

try {
    $result = $apiInstance->replaceTenantPackage($tenant_id, $id, $replace_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->replaceTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]