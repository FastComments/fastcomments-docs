## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Нет |  |
| afterCreatedAt | integer | query | Нет |  |
| unreadOnly | boolean | query | Нет |  |
| dmOnly | boolean | query | Нет |  |
| noDm | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Returns: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## Пример

[inline-code-attrs-start title = 'resetUserNotifications Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный HTTP‑клиент, передайте свой клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // строка
$options = [
    'after_id' => 'after_id_example', // строка
    'after_created_at' => 56, // целое
    'unread_only' => True, // булево
    'dm_only' => True, // булево
    'no_dm' => True, // булево
    'sso' => 'sso_example', // строка
];


try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]