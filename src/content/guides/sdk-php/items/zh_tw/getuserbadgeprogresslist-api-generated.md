## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## 回應

返回：[`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgeProgressListResponse.php)

## 範例

[inline-code-attrs-start title = 'getUserBadgeProgressList 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 配置 API 金鑰授權：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消註解以下代碼以設定 API 金鑰前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您想使用自訂的 HTTP 客戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// 字串
$options = [
    'user_id' => 'user_id_example', // string
    // 字串
    'limit' => 3.4, // float
    // 浮點數
    'skip' => 3.4, // float
    // 浮點數
];


try {
    $result = $apiInstance->getUserBadgeProgressList($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressList: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---