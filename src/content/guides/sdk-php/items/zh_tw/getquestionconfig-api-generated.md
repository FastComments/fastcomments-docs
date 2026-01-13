## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionConfig200Response.php)

## 範例

[inline-code-attrs-start title = 'getQuestionConfig 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API 金鑰授權：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消註解下列程式以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 此為可選，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字串
$id = 'id_example'; // 字串

try {
    $result = $apiInstance->getQuestionConfig($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---