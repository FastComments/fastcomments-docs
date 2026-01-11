## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| updateComments | boolean | query | 否 |  |

## 回應

回傳：[`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PutSSOUserAPIResponse.php)

## 範例

[inline-code-attrs-start title = 'putSSOUser 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如果需要，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 http client，請傳入實作 `GuzzleHttp\ClientInterface` 的 client。
    // 此為可選項，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_apisso_user_data = new \FastComments\Client\Model\UpdateAPISSOUserData(); // \FastComments\Client\Model\UpdateAPISSOUserData
$update_comments = True; // bool

try {
    $result = $apiInstance->putSSOUser($tenant_id, $id, $update_apisso_user_data, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->putSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---