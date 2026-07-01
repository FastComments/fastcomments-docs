## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Ответ

Возвращает: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadges'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройте авторизацию ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // строка
$options = [
    'user_id' => 'user_id_example', // строка
    'badge_id' => 'badge_id_example', // строка
    'type' => 3.4, // float
    'displayed_on_comments' => True, // bool
    'limit' => 3.4, // float
    'skip' => 3.4, // float
];


try {
    $result = $apiInstance->getUserBadges($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Исключение при вызове DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]