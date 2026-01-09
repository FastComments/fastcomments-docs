## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadgeProgressList200Response.php)

## 例

[inline-code-attrs-start title = 'getUserBadgeProgressList の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を構成します: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには以下のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface`を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 文字列
$user_id = 'user_id_example'; // 文字列
$limit = 3.4; // 浮動小数点数
$skip = 3.4; // 浮動小数点数

try {
    $result = $apiInstance->getUserBadgeProgressList($tenant_id, $user_id, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressList: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]