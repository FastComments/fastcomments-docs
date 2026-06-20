## 參數

| 名稱 | 型別 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## 回應

回傳：[`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultsResponse.php)

## 範例

[inline-code-attrs-start title = 'getQuestionResults 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如需為 API 金鑰設定前綴（例如 Bearer），請取消註解下方程式碼
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 http 客戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這為選用項，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字串
$url_id = 'url_id_example'; // 字串
$user_id = 'user_id_example'; // 字串
$start_date = 'start_date_example'; // 字串
$question_id = 'question_id_example'; // 字串
$question_ids = 'question_ids_example'; // 字串
$skip = 3.4; // 浮點數

try {
    $result = $apiInstance->getQuestionResults($tenant_id, $url_id, $user_id, $start_date, $question_id, $question_ids, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]