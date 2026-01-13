## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| urlId | string | query | いいえ |  |
| fromCommentId | string | query | いいえ |  |
| viewed | boolean | query | いいえ |  |
| type | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotifications200Response.php)

## 例

[inline-code-attrs-start title = 'getNotifications の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには以下の行のコメントを解除してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムHTTPクライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$url_id = 'url_id_example'; // string
$from_comment_id = 'from_comment_id_example'; // string
$viewed = True; // bool
$type = 'type_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getNotifications($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]