請遵循[安裝程序](#installation--usage)，然後執行以下內容：

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Configure API key authorization: api_key
// 設定 API 金鑰授權: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 若需要，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$tenant_id = 'tenant_id_example'; // 字串
$add_domain_config_params = new \FastComments\Client\Model\AddDomainConfigParams(); // \FastComments\Client\Model\AddDomainConfigParams

try {
    $result = $apiInstance->addDomainConfig($tenant_id, $add_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addDomainConfig: ', $e->getMessage(), PHP_EOL;
}

```