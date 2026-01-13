## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## 戻り値

戻り値: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplate200Response.php)

## 例

[inline-code-attrs-start title = 'getEmailTemplate の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// APIキー認証の設定: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要に応じて、APIキーにプレフィックス（例: Bearer）を設定するには、以下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムのHTTPクライアントを使用する場合、`GuzzleHttp\ClientInterface` を実装しているクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getEmailTemplate($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]