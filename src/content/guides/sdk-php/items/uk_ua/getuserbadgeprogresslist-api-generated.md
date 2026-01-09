## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadgeProgressList200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadgeProgressList'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації ключем API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadgeProgressList($tenant_id, $user_id, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressList: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]