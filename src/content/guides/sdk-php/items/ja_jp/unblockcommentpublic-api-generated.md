## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| commentId | string | パス | はい |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

戻り値: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UnblockSuccess.php)

## 例

[inline-code-attrs-start title = 'unBlockCommentPublic の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$public_block_from_comment_params = new \FastComments\Client\Model\PublicBlockFromCommentParams(); // \FastComments\Client\Model\PublicBlockFromCommentParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->unBlockCommentPublic($tenant_id, $comment_id, $public_block_from_comment_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->unBlockCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---