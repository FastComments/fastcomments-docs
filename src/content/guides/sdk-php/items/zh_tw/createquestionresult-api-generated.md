## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

傳回: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionResultResponse.php)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 設定 API 金鑰授權：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如有需要，取消註解下方程式以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 若要使用自訂的 http client，請傳入實作了 `GuzzleHttp\ClientInterface` 的 client。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_question_result_body = new \FastComments\Client\Model\CreateQuestionResultBody(); // \FastComments\Client\Model\CreateQuestionResultBody

try {
    $result = $apiInstance->createQuestionResult($tenant_id, $create_question_result_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]