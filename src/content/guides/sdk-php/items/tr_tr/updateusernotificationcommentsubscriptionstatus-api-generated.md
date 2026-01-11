Belirli bir yorum için bildirimleri etkinleştir veya devre dışı bırak.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| notificationId | string | path | Evet |  |
| optedInOrOut | string | path | Evet |  |
| commentId | string | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatus200Response.php)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
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