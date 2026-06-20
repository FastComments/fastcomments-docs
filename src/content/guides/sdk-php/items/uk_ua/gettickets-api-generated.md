## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| state | number | query | Ні |  |
| skip | number | query | Ні |  |
| limit | number | query | Ні |  |

## Відповідь

Повертає: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTicketsResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getTickets'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації API-ключем: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // рядок
$user_id = 'user_id_example'; // рядок
$state = 3.4; // число з плаваючою комою
$skip = 3.4; // число з плаваючою комою
$limit = 3.4; // число з плаваючою комою

try {
    $result = $apiInstance->getTickets($tenant_id, $user_id, $state, $skip, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---