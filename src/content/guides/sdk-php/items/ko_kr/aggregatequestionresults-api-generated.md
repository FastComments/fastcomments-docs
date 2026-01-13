## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니요 |  |
| questionIds | array | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| timeBucket | string | query | 아니요 |  |
| startDate | string | query | 아니요 |  |
| forceRecalculate | boolean | query | 아니요 |  |

## 응답

반환: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResults200Response.php)

## 예제

[inline-code-attrs-start title = 'aggregateQuestionResults 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API 키 인증 구성: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 필요한 경우 아래의 주석을 해제하여 API 키에 대한 접두사(예: Bearer)를 설정하세요
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
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