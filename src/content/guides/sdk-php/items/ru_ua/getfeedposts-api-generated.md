req
tenantId
afterId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Нет |  |
| limit | integer | query | Нет |  |
| tags | array | query | Нет |  |

## Ответ

Возвращает: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPosts200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getFeedPosts'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настроить авторизацию ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать кастомный HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // строка
$after_id = 'after_id_example'; // строка
$limit = 56; // целое
$tags = array('tags_example'); // массив строк

try {
    $result = $apiInstance->getFeedPosts($tenant_id, $after_id, $limit, $tags);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getFeedPosts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]