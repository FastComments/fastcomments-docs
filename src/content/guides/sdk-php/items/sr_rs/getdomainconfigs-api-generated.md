---
## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfigs200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример за getDomainConfigs'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментаришите ред испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите користити прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционo, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string

try {
    $result = $apiInstance->getDomainConfigs($tenant_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getDomainConfigs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---