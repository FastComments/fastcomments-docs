## Параметры

| Имя | Тип | Расположение | Требуется | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Ответ

Возвращает: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTicketsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getTickets'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройте авторизацию ключа API: api_key
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, при необходимости
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать кастомный HTTP-клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // строка
$options = [
    'user_id' => 'user_id_example', // строка
    'state' => 3.4, // число с плавающей запятой
    'skip' => 3.4, // число с плавающей запятой
    'limit' => 3.4, // число с плавающей запятой
];


try {
    $result = $apiInstance->getTickets($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]