## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## Ответ

Возвращает: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CombineCommentsWithQuestionResults200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример combineCommentsWithQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (e.g. Bearer) для API-ключа, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
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