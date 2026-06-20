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

Возвращает: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$after_id = 'after_id_example'; // строка
$after_created_at = 56; // целое
$unread_only = True; // логическое
$dm_only = True; // логическое
$no_dm = True; // логическое
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $after_id, $after_created_at, $unread_only, $dm_only, $no_dm, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]