## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionConfigResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за createQuestionConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на упълномощаване чрез API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу за да настроите префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате собствен HTTP клиент, подайте ваш клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще бъде използван по подразбиране.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_question_config_body = new \FastComments\Client\Model\CreateQuestionConfigBody(); // \FastComments\Client\Model\CreateQuestionConfigBody

try {
    $result = $apiInstance->createQuestionConfig($tenant_id, $create_question_config_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createQuestionConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]