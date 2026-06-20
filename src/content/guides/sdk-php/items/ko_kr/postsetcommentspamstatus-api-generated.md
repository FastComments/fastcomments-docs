## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| spam | boolean | query | 아니오 |  |
| permNotSpam | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 예제

[inline-code-attrs-start title = 'postSetCommentSpamStatus 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // 문자열
$spam = True; // 부울
$perm_not_spam = True; // 부울
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->postSetCommentSpamStatus($comment_id, $spam, $perm_not_spam, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentSpamStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]