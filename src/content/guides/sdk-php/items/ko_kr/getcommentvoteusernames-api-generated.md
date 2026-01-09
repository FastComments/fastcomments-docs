## 매개변수

| 이름 | Type | Location | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| dir | integer | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentVoteUserNames200Response.php)

## 예제

[inline-code-attrs-start title = 'getCommentVoteUserNames 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현한 클라이언트를 전달하세요.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$comment_id = 'comment_id_example'; // 문자열
$dir = 56; // 정수
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->getCommentVoteUserNames($tenant_id, $comment_id, $dir, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentVoteUserNames: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---