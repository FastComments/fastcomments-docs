특정 댓글에 대한 알림을 활성화하거나 비활성화합니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 쿼리 | 예 |  |
| notificationId | string | 경로 | 예 |  |
| optedInOrOut | string | 경로 | 예 |  |
| commentId | string | 쿼리 | 예 |  |
| sso | string | 쿼리 | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatus200Response.php)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$notification_id = 'notification_id_example'; // 문자열
$opted_in_or_out = 'opted_in_or_out_example'; // 문자열
$comment_id = 'comment_id_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->updateUserNotificationCommentSubscriptionStatus($tenant_id, $notification_id, $opted_in_or_out, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationCommentSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---