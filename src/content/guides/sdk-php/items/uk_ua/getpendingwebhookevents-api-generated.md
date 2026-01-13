## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | query | Ні |  |
| externalId | string | query | Ні |  |
| eventType | string | query | Ні |  |
| type | string | query | Ні |  |
| domain | string | query | Ні |  |
| attemptCountGT | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEvents200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getPendingWebhookEvents'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію за допомогою API-ключа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте рядок нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо це потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково — за замовчуванням буде використовуватися `GuzzleHttp\Client`.
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

---