Агрегирует документы путем их группировки (если указан groupBy) и применения нескольких операций.
Поддерживаются различные операции (например sum, countDistinct, avg и т.д.).

## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| parentTenantId | string | query | Нет |  |
| includeStats | boolean | query | Нет |  |

## Ответ

Возвращает: [`AggregationResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregationResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример aggregate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации API-ключом: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например Bearer) для API-ключа, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно: по умолчанию будет использован `GuzzleHttp\Client`.
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

---