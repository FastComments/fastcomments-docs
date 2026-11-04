페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새로운 루트 댓글에 대해 알림이 생성되며, 또한

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 예 |  |
| url | string | query | 예 |  |
| pageTitle | string | query | 예 |  |
| subscribedOrUnsubscribed | string | path | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환값: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationPageSubscriptionStatusResponse.php)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 http 클라이언트를 사용하려면, `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$url = 'url_example'; // string
$page_title = 'page_title_example'; // string
$subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example'; // string
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->updateUserNotificationPageSubscriptionStatus($tenant_id, $url_id, $url, $page_title, $subscribed_or_unsubscribed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]