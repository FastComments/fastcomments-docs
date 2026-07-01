## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Відповідь

Повертає: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModeratorsResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerators'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// // Налаштувати авторизацію API ключа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// // Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
// If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
// // Якщо ви хочете використовувати кастомний HTTP‑клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
// This is optional, `GuzzleHttp\Client` will be used as default.
// // Це необов’язково, `GuzzleHttp\Client` буде використано за замовчуванням.

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// // рядок
$skip = 3.4; // float
// // float


try {
    $result = $apiInstance->getModerators($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerators: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]