## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| broadcastId | string | query | いいえ |  |
| isLive | boolean | query | いいえ |  |
| doSpamCheck | boolean | query | いいえ |  |
| skipDupCheck | boolean | query | いいえ |  |

## レスポンス

返却: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostsResponse.php)

## 例

[inline-code-attrs-start title = 'createFeedPost の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API キー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、以下のコメントを解除して API キーの接頭辞（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 型: string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // 型: \FastComments\Client\Model\CreateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // 型: string
$is_live = True; // 型: bool
$do_spam_check = True; // 型: bool
$skip_dup_check = True; // 型: bool

try {
    $result = $apiInstance->createFeedPost($tenant_id, $create_feed_post_params, $broadcast_id, $is_live, $do_spam_check, $skip_dup_check);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createFeedPost: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]