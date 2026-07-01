## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplateRenderErrorsResponse.php)

## Приклад

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, `GuzzleHttp\Client` буде використано за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // рядок
$id = 'id_example'; // рядок
$skip = 3.4; // число з плаваючою крапкою


try {
    $result = $apiInstance->getEmailTemplateRenderErrors($tenant_id, $id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplateRenderErrors: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]