Currently-online viewers of a page: 페이지의 현재 웹소켓 세션이 구독 중인 사용자입니다.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Returns anonCount + totalCount (방 전체 구독자 수, 열거하지 않는 익명 뷰어 포함).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리된). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달합니다. |
| afterUserId | string | query | No | 커서 동점 해결: 이전 응답의 nextAfterUserId을 전달합니다. afterName이 설정된 경우 이름이 동일할 때 항목이 누락되지 않도록 필요합니다. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers 예시'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 페이지 URL 식별자 (서버 측에서 정리된).
$options = [
    'after_name' => 'after_name_example', // string | 커서: 이전 응답의 nextAfterName을 전달합니다.
    'after_user_id' => 'after_user_id_example', // string | 커서 동점 해결: 이전 응답의 nextAfterUserId을 전달합니다. afterName이 설정된 경우 이름이 동일할 때 항목이 누락되지 않도록 필요합니다.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]