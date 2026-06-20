## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| voteId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## 示例

[inline-code-attrs-start title = 'deleteModerationVote 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果要使用自定义 HTTP 客户端，传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteModerationVote($comment_id, $vote_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->deleteModerationVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---