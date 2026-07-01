## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## レスポンス

戻り値: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteDomainConfigResponse.php)

## 例

[inline-code-attrs-start title = 'deleteDomainConfig の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// APIキー認証を設定: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要に応じて、以下のコメントを外してAPIキーのプレフィックス（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムHTTPクライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これはオプショナルです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// 文字列
$domain = 'domain_example'; // string
// 文字列


try {
    $result = $apiInstance->deleteDomainConfig($tenant_id, $domain);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]