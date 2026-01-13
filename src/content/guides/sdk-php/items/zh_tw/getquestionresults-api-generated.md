## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| questionId | string | query | 否 |  |
| questionIds | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResults200Response.php)

## 範例

[inline-code-attrs-start title = 'getQuestionResults 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 密鑰授權: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如有需要，取消註解以下以為 API 金鑰設定前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果想使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$user_id = 'user_id_example'; // string
$start_date = 'start_date_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = 'question_ids_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getQuestionResults($tenant_id, $url_id, $user_id, $start_date, $question_id, $question_ids, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---