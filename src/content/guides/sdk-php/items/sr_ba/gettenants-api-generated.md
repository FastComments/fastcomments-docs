## Параметри

| Име | Тип | Location | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| meta | string | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantsResponse.php)

## Пример

[inline-code-attrs-start title = 'getTenants Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите ауторизацију помоћу API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Ако је потребно, откоментирајте испод да подесите префикс (нпр. Bearer) за API кључ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите користити прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално; као подразумевани ће се користити `GuzzleHttp\Client`.
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