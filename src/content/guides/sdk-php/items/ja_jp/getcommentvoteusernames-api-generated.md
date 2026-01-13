## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| dir | integer | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentVoteUserNames200Response.php)

## 例

[inline-code-attrs-start title = 'getCommentVoteUserNames の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 文字列
$comment_id = 'comment_id_example'; // 文字列
$dir = 56; // 整数
$sso = 'sso_example'; // 文字列

try {
    $result = $apiInstance->getCommentVoteUserNames($tenant_id, $comment_id, $dir, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentVoteUserNames: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---