## パラメータ

| Name | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddHashTagsBulk200Response.php)

## 例

[inline-code-attrs-start title = 'addHashTagsBulk の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API キー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには以下のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です、`GuzzleHttp\Client` がデフォルトで使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$bulk_create_hash_tags_body = new \FastComments\Client\Model\BulkCreateHashTagsBody(); // \FastComments\Client\Model\BulkCreateHashTagsBody

try {
    $result = $apiInstance->addHashTagsBulk($tenant_id, $bulk_create_hash_tags_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTagsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]