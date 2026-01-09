## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| voteId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| broadcastId | string | query | 是 |  |
| editKey | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentVote200Response.php)

## 示例

[inline-code-attrs-start title = 'deleteCommentVote 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]