---
## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |

## 回應

回傳: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantUserResponse.php)

## 範例

[inline-code-attrs-start title = 'createTenantUser 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 若需要，取消註解以下以為 API 金鑰設定前綴 (例如 Bearer)
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若想使用自訂的 http client，傳入實作 `GuzzleHttp\ClientInterface` 的 client。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_user_body = new \FastComments\Client\Model\CreateTenantUserBody(); // \FastComments\Client\Model\CreateTenantUserBody

try {
    $result = $apiInstance->createTenantUser($tenant_id, $create_tenant_user_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---