## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| badgeId | string | query | いいえ |  |
| type | number | query | いいえ |  |
| displayedOnComments | boolean | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadges200Response.php)

## 例

[inline-code-attrs-start title = 'getUserBadges の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーにプレフィックス（例: Bearer）を設定するには以下のコメントを解除してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface`を実装したクライアントを渡してください。
    // これは任意です。デフォルトでは`GuzzleHttp\Client`が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$badge_id = 'badge_id_example'; // string
$type = 3.4; // float
$displayed_on_comments = True; // bool
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadges($tenant_id, $user_id, $badge_id, $type, $displayed_on_comments, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]