## パラメータ

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|------|-----|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| badgeId | string | query | いいえ |  |
| type | number | query | いいえ |  |
| displayedOnComments | boolean | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

Returns: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgesResponse.php)

## 例

[inline-code-attrs-start title = 'getUserBadges の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// API キー認証を設定: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要に応じて、API キーのプレフィックス（例: Bearer）を設定する場合は以下のコメントを外してください
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 文字列
$options = [
    'user_id' => 'user_id_example', // 文字列
    'badge_id' => 'badge_id_example', // 文字列
    'type' => 3.4, // 浮動小数点数
    'displayed_on_comments' => True, // 真偽値
    'limit' => 3.4, // 浮動小数点数
    'skip' => 3.4, // 浮動小数点数
];


try {
    $result = $apiInstance->getUserBadges($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]