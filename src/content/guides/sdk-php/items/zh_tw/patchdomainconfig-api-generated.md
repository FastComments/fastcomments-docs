## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| domainToUpdate | string | path | 是 |  |

## 回應

回傳：[`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfig200Response.php)

## 範例

[inline-code-attrs-start title = 'patchDomainConfig 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您要使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 此為選用，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain_to_update = 'domain_to_update_example'; // string
$patch_domain_config_params = new \FastComments\Client\Model\PatchDomainConfigParams(); // \FastComments\Client\Model\PatchDomainConfigParams

try {
    $result = $apiInstance->patchDomainConfig($tenant_id, $domain_to_update, $patch_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]