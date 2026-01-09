## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddSSOUserAPIResponse.php)

## 例

[inline-code-attrs-start title = 'addSSOUser の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、以下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_apisso_user_data = new \FastComments\Client\Model\CreateAPISSOUserData(); // \FastComments\Client\Model\CreateAPISSOUserData

try {
    $result = $apiInstance->addSSOUser($tenant_id, $create_apisso_user_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]