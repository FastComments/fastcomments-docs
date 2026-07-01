## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

返回: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddSSOUserAPIResponse.php)

## 範例

[inline-code-attrs-start title = 'addSSOUser 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 取消註解以下以設定 API 金鑰前綴（例如 Bearer），如有需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自訂的 HTTP 客戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設將使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 字串
$create_apisso_user_data = new \FastComments\Client\Model\CreateAPISSOUserData(); // \FastComments\Client\Model\CreateAPISSOUserData


try {
    $result = $apiInstance->addSSOUser($tenant_id, $create_apisso_user_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---