## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionConfigs200Response.php)

## 例

[inline-code-attrs-start title = 'getQuestionConfigs の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API キー認証を設定します: api_key
// 必要に応じて、API キーに接頭辞（例: Bearer）を設定するには以下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは省略可能で、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getQuestionConfigs($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionConfigs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]