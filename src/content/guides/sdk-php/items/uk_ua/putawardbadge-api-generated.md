## Parameters

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Response

Повертає: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AwardUserBadgeResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад putAwardBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати кастомний HTTP‑клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$badge_id = 'badge_id_example'; // рядок
$options = [
    'user_id' => 'user_id_example', // рядок
    'comment_id' => 'comment_id_example', // рядок
    'broadcast_id' => 'broadcast_id_example', // рядок
    'sso' => 'sso_example', // рядок
];


try {
    $result = $apiInstance->putAwardBadge($tenant_id, $badge_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Виключення при виклику ModerationApi->putAwardBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]