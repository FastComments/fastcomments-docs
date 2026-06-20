## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| sure | string | query | 否 |  |

## 回應

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 範例

[inline-code-attrs-start title = 'deleteTenant 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API 金鑰授權: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 若需要，解除註解下列程式以設定前置詞（例如 Bearer）給 API 金鑰


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 若要使用自訂的 http 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$sure = 'sure_example'; // string

try {
    $result = $apiInstance->deleteTenant($tenant_id, $id, $sure);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteTenant: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]