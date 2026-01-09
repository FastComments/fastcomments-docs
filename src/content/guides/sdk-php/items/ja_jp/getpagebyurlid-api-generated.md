## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| urlId | string | クエリ | はい |  |

## レスポンス

戻り値: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPageByURLIdAPIResponse.php)

## 例

[inline-code-attrs-start title = 'getPageByURLId の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて以下のコメントを外してAPIキーの接頭辞（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string（文字列）
$url_id = 'url_id_example'; // string（文字列）

try {
    $result = $apiInstance->getPageByURLId($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPageByURLId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---