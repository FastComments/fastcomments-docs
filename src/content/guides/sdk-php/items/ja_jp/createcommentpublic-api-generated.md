## パラメータ

| 名前 | 種類 | 場所 | 必須 | 説明 |
|------|------|----------|------|------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| broadcastId | string | query | はい |  |
| sessionId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

Returns: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveCommentsResponseWithPresence.php)

## 例

[inline-code-attrs-start title = 'createCommentPublic の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 文字列
$url_id = 'url_id_example'; // 文字列
$broadcast_id = 'broadcast_id_example'; // 文字列
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$options = [
    'session_id' => 'session_id_example', // 文字列
    'sso' => 'sso_example', // 文字列
];


try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---