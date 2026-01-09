ドキュメントをグループ化（groupBy が提供されている場合）し、複数の操作を適用して集計します。sum、countDistinct、avg などのさまざまな操作がサポートされています。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| parentTenantId | string | クエリ | いいえ |  |
| includeStats | boolean | クエリ | いいえ |  |

## レスポンス

戻り値: [`AggregationResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregationResponse.php)

## 例

[inline-code-attrs-start title = 'aggregate の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API キー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、API キー用のプレフィックス（例: Bearer）を設定するには下の行のコメントを解除してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$aggregation_request = new \FastComments\Client\Model\AggregationRequest(); // \FastComments\Client\Model\AggregationRequest
$parent_tenant_id = 'parent_tenant_id_example'; // string
$include_stats = True; // bool

try {
    $result = $apiInstance->aggregate($tenant_id, $aggregation_request, $parent_tenant_id, $include_stats);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---