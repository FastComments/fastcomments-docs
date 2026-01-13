## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## 例

[inline-code-attrs-start title = 'updateTenant の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装しているクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_tenant_body = new \FastComments\Client\Model\UpdateTenantBody(); // \FastComments\Client\Model\UpdateTenantBody

try {
    $result = $apiInstance->updateTenant($tenant_id, $id, $update_tenant_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateTenant: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]