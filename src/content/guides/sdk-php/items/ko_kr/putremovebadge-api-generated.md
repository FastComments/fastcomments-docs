## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | 예 |  |
| userId | string | query | 아니오 |  |
| commentId | string | query | 아니오 |  |
| broadcastId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RemoveUserBadgeResponse.php)

## 예제

[inline-code-attrs-start title = 'putRemoveBadge 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려는 경우, `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이 항목은 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // 문자열
$user_id = 'user_id_example'; // 문자열
$comment_id = 'comment_id_example'; // 문자열
$broadcast_id = 'broadcast_id_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->putRemoveBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putRemoveBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]