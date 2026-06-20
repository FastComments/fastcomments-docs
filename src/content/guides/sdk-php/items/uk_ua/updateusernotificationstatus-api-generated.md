## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| notificationId | string | path | Так |  |
| newStatus | string | path | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatusResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад використання updateUserNotificationStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використовуватися `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$notification_id = 'notification_id_example'; // string
$new_status = 'new_status_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->updateUserNotificationStatus($tenant_id, $notification_id, $new_status, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]