## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionConfigs200Response.php)

## Пример

[inline-code-attrs-start title = 'getQuestionConfigs Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите ауторизацију API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментирајте доле да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getQuestionConfigs($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionConfigs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]