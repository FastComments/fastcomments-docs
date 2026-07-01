## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| commentId | string | query | Ні |  |
| externalId | string | query | Ні |  |
| eventType | string | query | Ні |  |
| type | string | query | Ні |  |
| domain | string | query | Ні |  |
| attemptCountGT | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventsResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getPendingWebhookEvents'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// // Налаштування авторизації ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// // Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // // Якщо ви хочете використати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // // Це необов'язково, `GuzzleHttp\Client` буде використано за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // рядок
$options = [
    'comment_id' => 'comment_id_example', // рядок
    'external_id' => 'external_id_example', // рядок
    'event_type' => 'event_type_example', // рядок
    'type' => 'type_example', // рядок
    'domain' => 'domain_example', // рядок
    'attempt_count_gt' => 3.4, // число з плаваючою крапкою
    'skip' => 3.4, // число з плаваючою крапкою
];


try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]