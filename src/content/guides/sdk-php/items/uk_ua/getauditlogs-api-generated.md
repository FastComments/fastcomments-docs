## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |
| order | string | query | Ні |  |
| after | number | query | Ні |  |
| before | number | query | Ні |  |

## Відповідь

Повертає: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogsResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getAuditLogs'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації API-ключем: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використати кастомний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, буде використано `GuzzleHttp\Client` за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float
$order = new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(); // \FastComments\Client\Model\SORTDIR
$after = 3.4; // float
$before = 3.4; // float

try {
    $result = $apiInstance->getAuditLogs($tenant_id, $limit, $skip, $order, $after, $before);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---