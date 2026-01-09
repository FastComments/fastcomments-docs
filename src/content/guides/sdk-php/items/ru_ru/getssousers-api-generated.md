## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | integer | query | Нет |  |

## Ответ

Возвращает: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUsers200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getSSOUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте строку ниже, чтобы задать префикс (например, Bearer) для API-ключа, если это необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, `GuzzleHttp\Client` будет использован как значение по умолчанию.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 56; // int

try {
    $result = $apiInstance->getSSOUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]