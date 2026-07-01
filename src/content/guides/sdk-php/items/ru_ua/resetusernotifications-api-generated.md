## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| afterId | string | query | Ні |  |
| afterCreatedAt | integer | query | Ні |  |
| unreadOnly | boolean | query | Ні |  |
| dmOnly | boolean | query | Ні |  |
| noDm | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад resetUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використати власний HTTP‑клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$options = [
    'after_id' => 'after_id_example', // рядок
    'after_created_at' => 56, // ціле
    'unread_only' => True, // булевий
    'dm_only' => True, // булевий
    'no_dm' => True, // булевий
    'sso' => 'sso_example', // рядок
];


try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]