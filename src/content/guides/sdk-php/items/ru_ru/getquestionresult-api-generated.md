## Параметры

| Название | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResult200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResult'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getQuestionResult($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]