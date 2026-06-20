## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| direction | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## 例

[inline-code-attrs-start title = 'postVote の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$direction = 'direction_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postVote($comment_id, $direction, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]