## Параметри

| Име | Type | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |

## Отговор

Връща: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventCount200Response.php)

## Пример

[inline-code-attrs-start title = 'getPendingWebhookEventCount Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигуриране на оторизация с API ключ: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако искате да използвате персонализиран HTTP клиент, подайте ваш клиент, който реализира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
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

try {
    $result = $apiInstance->getPendingWebhookEventCount($tenant_id, $comment_id, $external_id, $event_type, $type, $domain, $attempt_count_gt);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEventCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]