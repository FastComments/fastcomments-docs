## Параметри

| Ім'я | Тип | Місцезнаходження | Обов'язково | Опис |
|------|------|-------------------|--------------|------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Відповідь

Возвращает: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateQuestionResultsResponse.php)

## Приклад

[inline-code-attrs-start title = 'aggregateQuestionResults Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментировать ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать кастомный HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'question_id' => 'question_id_example', // string
    'question_ids' => array('question_ids_example'), // string[]
    'url_id' => 'url_id_example', // string
    'time_bucket' => new \FastComments\Client\Model\\FastComments\Client\Model\AggregateTimeBucket(), // \FastComments\Client\Model\AggregateTimeBucket
    'start_date' => new \DateTime('2013-10-20T19:20:30+01:00'), // \DateTime
    'force_recalculate' => True, // bool
];


try {
    $result = $apiInstance->aggregateQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Исключение при вызове DefaultApi->aggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---