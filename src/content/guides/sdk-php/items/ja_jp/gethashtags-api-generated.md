## パラメーター

| Name | Type | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | number | query | いいえ |  |

## レスポンス

戻り値: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetHashTagsResponse.php)

## 例

[inline-code-attrs-start title = 'getHashTags の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要ならAPIキーのプレフィックス（例: Bearer）を設定するために以下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用したい場合、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getHashTags($tenant_id, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getHashTags: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]