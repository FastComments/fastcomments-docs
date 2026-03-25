## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| state | number | query | 否 |  |
| skip | number | query | 否 |  |
| limit | number | query | 否 |  |

## 回應

回傳: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTickets200Response.php)

## 範例

[inline-code-attrs-start title = 'getTickets 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如需為 API 金鑰設定前綴（例如 Bearer），請取消註解下方
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是選用的，如未提供預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$state = 3.4; // float
$skip = 3.4; // float
$limit = 3.4; // float

try {
    $result = $apiInstance->getTickets($tenant_id, $user_id, $state, $skip, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]