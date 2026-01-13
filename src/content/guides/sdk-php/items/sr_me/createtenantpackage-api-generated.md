## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantPackage200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример createTenantPackage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Ово је опционално, подразумевано ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_package_body = new \FastComments\Client\Model\CreateTenantPackageBody(); // \FastComments\Client\Model\CreateTenantPackageBody

try {
    $result = $apiInstance->createTenantPackage($tenant_id, $create_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]