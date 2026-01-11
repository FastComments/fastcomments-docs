Включить или отключить уведомления для конкретного комментария.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| notificationId | string | path | Да |  |
| optedInOrOut | string | path | Да |  |
| commentId | string | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatus200Response.php)

## Пример

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$notification_id = 'notification_id_example'; // string
$opted_in_or_out = 'opted_in_or_out_example'; // string
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->updateUserNotificationCommentSubscriptionStatus($tenant_id, $notification_id, $opted_in_or_out, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationCommentSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]