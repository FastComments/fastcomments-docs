## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| commentId | string | 路徑 | 是 |  |
| voteId | string | 路徑 | 是 |  |
| urlId | string | 查詢 | 是 |  |
| broadcastId | string | 查詢 | 是 |  |
| editKey | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## 範例

[inline-code-attrs-start title = 'deleteCommentVote 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設將使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 字串
$comment_id = 'comment_id_example'; // 字串
$vote_id = 'vote_id_example'; // 字串
$url_id = 'url_id_example'; // 字串
$broadcast_id = 'broadcast_id_example'; // 字串
$options = [
    'edit_key' => 'edit_key_example', // 字串
    'sso' => 'sso_example', // 字串
];


try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---