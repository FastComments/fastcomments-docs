## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| questionId | string | query | Hayır |  |
| questionIds | array | query | Hayır |  |
| urlId | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| forceRecalculate | boolean | query | Hayır |  |
| minValue | number | query | Hayır |  |
| maxValue | number | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CombineQuestionResultsWithCommentsResponse.php)

## Örnek

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// API anahtarı yetkilendirmesini yapılandır: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak amacıyla aşağıdakinin yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi iletin.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = array('question_ids_example'); // string[]
$url_id = 'url_id_example'; // string
$start_date = new \DateTime('2013-10-20T19:20:30+01:00'); // \DateTime
$force_recalculate = True; // bool
$min_value = 3.4; // float
$max_value = 3.4; // float
$limit = 3.4; // float

try {
    $result = $apiInstance->combineCommentsWithQuestionResults($tenant_id, $question_id, $question_ids, $url_id, $start_date, $force_recalculate, $min_value, $max_value, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->combineCommentsWithQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]