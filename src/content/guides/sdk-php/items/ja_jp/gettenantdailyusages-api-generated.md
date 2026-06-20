## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| yearNumber | number | query | いいえ |  |
| monthNumber | number | query | いいえ |  |
| dayNumber | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantDailyUsagesResponse.php)

## 例

[inline-code-attrs-start title = 'getTenantDailyUsages の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて APIキーの接頭辞（例: Bearer）を設定するには、以下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは省略可能です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$year_number = 3.4; // float
$month_number = 3.4; // float
$day_number = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantDailyUsages($tenant_id, $year_number, $month_number, $day_number, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantDailyUsages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]