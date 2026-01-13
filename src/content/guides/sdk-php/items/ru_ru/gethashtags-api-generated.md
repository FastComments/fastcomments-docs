## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |

## Ответ

Возвращает: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetHashTags200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getHashTags'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Настройка авторизации API ключа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API ключа, если требуется
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // строка
$page = 3.4; // число с плавающей точкой

try {
    $result = $apiInstance->getHashTags($tenant_id, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getHashTags: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]