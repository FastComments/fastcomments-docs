## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| broadcastId | string | query | 否 |  |
| isLive | boolean | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| skipDupCheck | boolean | query | 否 |  |

## 回應

回傳: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPost200Response.php)

## 範例

[inline-code-attrs-start title = 'createFeedPost 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如有需要，請取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 http client，請傳入實作了 `GuzzleHttp\ClientInterface` 的 client。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
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