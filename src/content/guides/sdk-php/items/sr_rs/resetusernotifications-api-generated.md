## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| afterId | string | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
// Ово је опционално, подразумевано ће се користити `GuzzleHttp\Client`.
$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$after_id = 'after_id_example'; // string
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $after_id, $after_created_at, $unread_only, $dm_only, $no_dm, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]