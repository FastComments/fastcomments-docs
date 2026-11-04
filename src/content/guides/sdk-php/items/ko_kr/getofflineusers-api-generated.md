현재 온라인 상태가 아닌, 페이지의 과거 댓글 작성자들입니다. displayName 기준으로 정렬됩니다.
/users/online을 모두 조회한 후 "멤버" 섹션을 렌더링할 때 이 엔드포인트를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버가 부분 인덱스 {tenantId, urlId, commenterName}을(를) afterName부터 $gt로 순회하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자(서버 측에서 정제됨). |
| afterName | string | query | 아니요 | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | 아니요 | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 필수이며, 이름이 같은 항목이 누락되지 않도록 합니다. |

## 응답

반환값: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 페이지 URL 식별자(서버 측에서 정제됨).
$options = [
    'after_name' => 'after_name_example', // string | 커서: 이전 응답의 nextAfterName을 전달하세요.
    'after_user_id' => 'after_user_id_example', // string | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 필수이며, 이름이 같은 항목이 누락되지 않도록 합니다.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]