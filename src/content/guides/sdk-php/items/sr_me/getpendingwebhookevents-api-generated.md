## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEvents200Response.php)

## Пример

[inline-code-attrs-start title = 'getPendingWebhookEvents Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите овлашћење помоћу API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментаришите доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите користити прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
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