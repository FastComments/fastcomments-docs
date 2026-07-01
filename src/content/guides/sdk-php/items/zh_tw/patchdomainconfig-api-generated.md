## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## 回應

返回：[`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchDomainConfigResponse.php)

## 範例

[inline-code-attrs-start title = 'patchDomainConfig 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 配置 API 金鑰授權：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消註解以下以設定 API 金鑰的前綴（例如 Bearer）

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 若要使用自訂 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 字串
$domain_to_update = 'domain_to_update_example'; // 字串
$patch_domain_config_params = new \FastComments\Client\Model\PatchDomainConfigParams(); // \FastComments\Client\Model\PatchDomainConfigParams


try {
    $result = $apiInstance->patchDomainConfig($tenant_id, $domain_to_update, $patch_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]