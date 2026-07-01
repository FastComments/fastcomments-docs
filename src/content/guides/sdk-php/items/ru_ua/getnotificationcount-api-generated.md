## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Ответ

Возвращает: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationCountResponse.php)

## Пример

[inline-code-attrs-start title = 'getNotificationCount Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройте авторизацию ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы настроить префикс (например, Bearer) для ключа API, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать собственный HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, `GuzzleHttp\Client` будет использоваться по умолчанию.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // строка
$options = [
    'user_id' => 'user_id_example', // строка
    'url_id' => 'url_id_example', // строка
    'from_comment_id' => 'from_comment_id_example', // строка
    'viewed' => True, // логическое
    'type' => 'type_example', // строка
];


try {
    $result = $apiInstance->getNotificationCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]