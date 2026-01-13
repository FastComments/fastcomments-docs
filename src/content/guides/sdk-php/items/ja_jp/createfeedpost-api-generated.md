## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| broadcastId | string | query | いいえ |  |
| isLive | boolean | query | いいえ |  |
| doSpamCheck | boolean | query | いいえ |  |
| skipDupCheck | boolean | query | いいえ |  |

## レスポンス

戻り値: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPost200Response.php)

## 例

[inline-code-attrs-start title = 'createFeedPost の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を構成: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要な場合は以下のコメントを外して、APIキーのプレフィックス（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用したい場合、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$is_live = True; // bool
$do_spam_check = True; // bool
$skip_dup_check = True; // bool

try {
    $result = $apiInstance->createFeedPost($tenant_id, $create_feed_post_params, $broadcast_id, $is_live, $do_spam_check, $skip_dup_check);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createFeedPost: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]