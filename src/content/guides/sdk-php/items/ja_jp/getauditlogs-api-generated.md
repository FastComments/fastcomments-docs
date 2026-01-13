## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| order | string | query | いいえ |  |
| after | number | query | いいえ |  |
| before | number | query | いいえ |  |

## レスポンス

戻り値: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogs200Response.php)

## 例

[inline-code-attrs-start title = 'getAuditLogs の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーにプレフィックス（例: Bearer）を設定するには以下のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float
$order = new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(); // \FastComments\Client\Model\SORTDIR
$after = 3.4; // float
$before = 3.4; // float

try {
    $result = $apiInstance->getAuditLogs($tenant_id, $limit, $skip, $order, $after, $before);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]