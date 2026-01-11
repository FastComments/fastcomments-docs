## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`PinComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PinComment200Response.php)

## 예제

[inline-code-attrs-start title = 'unPinComment 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$comment_id = 'comment_id_example'; // 문자열
$broadcast_id = 'broadcast_id_example'; // 문자열
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->unPinComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->unPinComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]