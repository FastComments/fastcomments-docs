## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Нет |  |
| questionIds | array | query | Нет |  |
| urlId | string | query | Нет |  |
| timeBucket | string | query | Нет |  |
| startDate | string | query | Нет |  |
| forceRecalculate | boolean | query | Нет |  |

## Ответ

Возвращает: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResultsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример aggregateQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации с помощью API-ключа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если это необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, `GuzzleHttp\Client` будет использоваться по умолчанию.
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