## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchPageAPIResponse.php)

## 例

[inline-code-attrs-start title = 'patchPage の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要なら以下のコメントを外して、APIキーのプレフィックス（例: Bearer）を設定します
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_api_page_data = new \FastComments\Client\Model\UpdateAPIPageData(); // \FastComments\Client\Model\UpdateAPIPageData

try {
    $result = $apiInstance->patchPage($tenant_id, $id, $update_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]