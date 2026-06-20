## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| postId | string | 경로 | 예 |  |
| broadcastId | string | 쿼리 | 아니오 |  |
| sso | string | 쿼리 | 아니오 |  |

## 응답

반환: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteFeedPostPublicResponse.php)

## 예제

[inline-code-attrs-start title = 'deleteFeedPostPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 지정 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며 기본값으로는 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteFeedPostPublic($tenant_id, $post_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---