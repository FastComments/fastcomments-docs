---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantUserResponse.php)

## Пример

[inline-code-attrs-start title = 'createTenantUser Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите овлашћење помоћу API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментирајте доле да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_user_body = new \FastComments\Client\Model\CreateTenantUserBody(); // \FastComments\Client\Model\CreateTenantUserBody

try {
    $result = $apiInstance->createTenantUser($tenant_id, $create_tenant_user_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---