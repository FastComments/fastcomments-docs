현재 온라인이 아닌 해당 페이지의 과거 댓글 작성자들입니다. displayName으로 정렬됩니다.
이것은 /users/online을 모두 확인한 후 "Members" 섹션을 렌더링하기 위해 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 인덱스 {tenantId, urlId, commenterName}를 afterName 이후부터 $gt를 통해 앞으로 탐색하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | No | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 동일한 항목이 누락되지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. `GuzzleHttp\Client`가 기본값으로 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 페이지 URL 식별자 (서버측에서 정리됨).
$after_name = 'after_name_example'; // string | 커서: 이전 응답의 nextAfterName을 전달하세요.
$after_user_id = 'after_user_id_example'; // string | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 동일한 항목이 누락되지 않도록 필요합니다.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]