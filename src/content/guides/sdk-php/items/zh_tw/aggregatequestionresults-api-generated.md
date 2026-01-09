## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| timeBucket | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |

## 回應

回傳：[`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResults200Response.php)

## 範例

[inline-code-attrs-start title = 'aggregateQuestionResults 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 若需要，取消註解下列以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = array('question_ids_example'); // string[]
$url_id = 'url_id_example'; // string
$time_bucket = new \FastComments\Client\Model\\FastComments\Client\Model\AggregateTimeBucket(); // \FastComments\Client\Model\AggregateTimeBucket
$start_date = new \DateTime('2013-10-20T19:20:30+01:00'); // \DateTime
$force_recalculate = True; // bool

try {
    $result = $apiInstance->aggregateQuestionResults($tenant_id, $question_id, $question_ids, $url_id, $time_bucket, $start_date, $force_recalculate);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]