## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateModerator200Response.php)

## 範例

[inline-code-attrs-start title = 'createModerator 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API key 授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 若需要，取消註解下列程式以設定前綴（例如 Bearer）到 API key
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 若要使用自訂的 http client，傳入實作了 `GuzzleHttp\ClientInterface` 的 client。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字串
$create_moderator_body = new \FastComments\Client\Model\CreateModeratorBody(); // \FastComments\Client\Model\CreateModeratorBody

try {
    $result = $apiInstance->createModerator($tenant_id, $create_moderator_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]