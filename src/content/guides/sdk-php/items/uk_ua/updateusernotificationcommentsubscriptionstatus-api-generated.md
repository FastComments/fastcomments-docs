Увімкнути або вимкнути сповіщення для конкретного коментаря.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| notificationId | string | path | Так |  |
| optedInOrOut | string | path | Так |  |
| commentId | string | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationCommentSubscriptionStatusResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад updateUserNotificationCommentSubscriptionStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використати кастомний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // рядок
$notification_id = 'notification_id_example'; // рядок
$opted_in_or_out = 'opted_in_or_out_example'; // рядок
$comment_id = 'comment_id_example'; // рядок
$sso = 'sso_example'; // рядок

try {
    $result = $apiInstance->updateUserNotificationCommentSubscriptionStatus($tenant_id, $notification_id, $opted_in_or_out, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationCommentSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]