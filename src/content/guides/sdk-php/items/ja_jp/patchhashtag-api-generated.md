## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tag | string | path | はい |  |
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchHashTag200Response.php)

## 例

[inline-code-attrs-start title = 'patchHashTag の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーにプレフィックス（例: Bearer）を設定するには下の行のコメントを解除してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface`を実装したクライアントを渡してください。
    // これはオプションです。デフォルトでは`GuzzleHttp\Client`が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // string
$tenant_id = 'tenant_id_example'; // string
$update_hash_tag_body = new \FastComments\Client\Model\UpdateHashTagBody(); // \FastComments\Client\Model\UpdateHashTagBody

try {
    $result = $apiInstance->patchHashTag($tag, $tenant_id, $update_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]