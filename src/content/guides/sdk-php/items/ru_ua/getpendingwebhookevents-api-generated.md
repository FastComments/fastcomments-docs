## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Нет |  |
| externalId | string | query | Нет |  |
| eventType | string | query | Нет |  |
| type | string | query | Нет |  |
| domain | string | query | Нет |  |
| attemptCountGT | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEvents200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getPendingWebhookEvents'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настроить авторизацию по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если нужно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$external_id = 'external_id_example'; // string
$event_type = 'event_type_example'; // string
$type = 'type_example'; // string
$domain = 'domain_example'; // string
$attempt_count_gt = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $comment_id, $external_id, $event_type, $type, $domain, $attempt_count_gt, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]