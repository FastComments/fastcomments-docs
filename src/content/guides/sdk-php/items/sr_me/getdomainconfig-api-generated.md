## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| domain | string | path | Да |  |

## Одговор

Враћа: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfigResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигурисање ауторизације API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментирајте доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain = 'domain_example'; // string

try {
    $result = $apiInstance->getDomainConfig($tenant_id, $domain);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]