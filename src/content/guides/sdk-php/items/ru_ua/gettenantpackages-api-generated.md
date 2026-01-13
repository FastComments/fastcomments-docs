## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackages200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getTenantPackages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если требуется
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantPackages($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantPackages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]