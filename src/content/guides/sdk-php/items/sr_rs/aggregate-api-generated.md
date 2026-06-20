Агрегира документе групишући их (ако је groupBy наведен) и примењујући више операција. Подржане су различите операције (нпр. sum, countDistinct, avg, итд.).

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Не |  |
| includeStats | boolean | query | Не |  |

## Одговор

Враћа: [`AggregateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример агрегирања'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Подесите овлашћење за API кључ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откомунтујте линију испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$aggregation_request = new \FastComments\Client\Model\AggregationRequest(); // \FastComments\Client\Model\AggregationRequest
$parent_tenant_id = 'parent_tenant_id_example'; // string
$include_stats = True; // bool

try {
    $result = $apiInstance->aggregate($tenant_id, $aggregation_request, $parent_tenant_id, $include_stats);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]