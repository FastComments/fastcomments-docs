## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## 예제

[inline-code-attrs-start title = 'resetUserNotificationCount 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->resetUserNotificationCount($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]