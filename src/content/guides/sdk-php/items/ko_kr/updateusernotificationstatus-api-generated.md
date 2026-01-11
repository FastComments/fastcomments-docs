## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| notificationId | string | path | 예 |  |
| newStatus | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatus200Response.php)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationStatus 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
$new_status = 'new_status_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->updateUserNotificationStatus($tenant_id, $notification_id, $new_status, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---