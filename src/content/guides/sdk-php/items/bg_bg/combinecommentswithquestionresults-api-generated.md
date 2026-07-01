## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Не |  |
| questionIds | array | query | Не |  |
| urlId | string | query | Не |  |
| startDate | string | query | Не |  |
| forceRecalculate | boolean | query | Не |  |
| minValue | number | query | Не |  |
| maxValue | number | query | Не |  |
| limit | number | query | Не |  |

## Отговор

Връща: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CombineQuestionResultsWithCommentsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за combineCommentsWithQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на удостоверяване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желаете да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е незадължително, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // низ
$options = [
    'question_id' => 'question_id_example', // низ
    'question_ids' => array('question_ids_example'), // низ[]
    'url_id' => 'url_id_example', // низ
    'start_date' => new \DateTime('2013-10-20T19:20:30+01:00'), // \DateTime
    'force_recalculate' => True, // булев
    'min_value' => 3.4, // десетично число
    'max_value' => 3.4, // десетично число
    'limit' => 3.4, // десетично число
];


try {
    $result = $apiInstance->combineCommentsWithQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->combineCommentsWithQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]