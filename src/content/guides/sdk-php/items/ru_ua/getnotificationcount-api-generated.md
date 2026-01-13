## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| urlId | string | query | Нет |  |
| fromCommentId | string | query | Нет |  |
| viewed | boolean | query | Нет |  |
| type | string | query | Нет |  |

## Ответ

Возвращает: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationCount200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Настройка авторизации по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если нужно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$url_id = 'url_id_example'; // string
$from_comment_id = 'from_comment_id_example'; // string
$viewed = True; // bool
$type = 'type_example'; // string

try {
    $result = $apiInstance->getNotificationCount($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]