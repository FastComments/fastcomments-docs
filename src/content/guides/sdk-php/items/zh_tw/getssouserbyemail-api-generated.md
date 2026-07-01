## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| email | string | path | Yes |  |

## Response

返回：[`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUserByEmailAPIResponse.php)

## 範例

[inline-code-attrs-start title = 'getSSOUserByEmail 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API 金鑰授權：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 取消註解以下程式碼以設定 API 金鑰的前綴（例如 Bearer），如有需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設將使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$email = 'email_example'; // string


try {
    $result = $apiInstance->getSSOUserByEmail($tenant_id, $email);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUserByEmail: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]