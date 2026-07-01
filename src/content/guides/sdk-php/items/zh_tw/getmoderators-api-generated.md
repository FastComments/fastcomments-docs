## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|------|------|------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 回應

Returns: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModeratorsResponse.php)

## 範例

[inline-code-attrs-start title = 'getModerators 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 解除註解以下以設定前綴（例如 Bearer）給 API 金鑰，如有需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果你想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設將使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float


try {
    $result = $apiInstance->getModerators($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerators: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---