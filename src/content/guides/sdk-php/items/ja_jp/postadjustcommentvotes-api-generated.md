## パラメータ

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|------|-------------|
| tenantId | string | クエリ | はい |  |
| commentId | string | パス | はい |  |
| broadcastId | string | クエリ | いいえ |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

返却: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AdjustVotesResponse.php)

## 例

[inline-code-attrs-start title = 'postAdjustCommentVotes の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 文字列
$comment_id = 'comment_id_example'; // 文字列
$adjust_comment_votes_params = new \FastComments\Client\Model\AdjustCommentVotesParams(); // \FastComments\Client\Model\AdjustCommentVotesParams
$options = [
    'broadcast_id' => 'broadcast_id_example', // 文字列
    'sso' => 'sso_example', // 文字列
];


try {
    $result = $apiInstance->postAdjustCommentVotes($tenant_id, $comment_id, $adjust_comment_votes_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postAdjustCommentVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]