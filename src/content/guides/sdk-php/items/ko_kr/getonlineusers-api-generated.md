현재 페이지에 접속해 있는 뷰어: 웹소켓 세션이 현재 해당 페이지를 구독하고 있는 사람들입니다.
익명 뷰어는 열거하지 않지만, anonCount + totalCount를 반환합니다 (룸 전체 구독자 수, 열거하지 않는 익명 뷰어 포함).

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | 아니오 | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | 아니오 | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 같은 항목이 누락되지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## 예제

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Page URL identifier (cleaned server-side).
$after_name = 'after_name_example'; // string | Cursor: pass nextAfterName from the previous response.
$after_user_id = 'after_user_id_example'; // string | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. Required when afterName is set so name-ties don't drop entries.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]