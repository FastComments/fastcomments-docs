## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tag | string | path | 是 |  |
| tenantId | string | query | 否 |  |

## 回應

回傳: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateHashTagResponse.php)

## 範例

[inline-code-attrs-start title = 'patchHashTag 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API 金鑰授權：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如需，取消註解下列以設定前綴（例如 Bearer）作為 API 金鑰
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 此為可選，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // string
$tenant_id = 'tenant_id_example'; // string
$update_hash_tag_body = new \FastComments\Client\Model\UpdateHashTagBody(); // \FastComments\Client\Model\UpdateHashTagBody

try {
    $result = $apiInstance->patchHashTag($tag, $tenant_id, $update_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]