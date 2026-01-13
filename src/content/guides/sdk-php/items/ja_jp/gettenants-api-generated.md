## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| meta | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenants200Response.php)

## 例

[inline-code-attrs-start title = 'getTenants の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要な場合は以下のコメントアウトを外して、APIキーのプレフィックス（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface`を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは`GuzzleHttp\Client`が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$meta = 'meta_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenants($tenant_id, $meta, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenants: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]