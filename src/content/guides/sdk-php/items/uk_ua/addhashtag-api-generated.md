## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ні |  |

## Відповідь

Повертає: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddHashTag200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад addHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації за допомогою API ключа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте наступний рядок, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати кастомний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використовуватись `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_hash_tag_body = new \FastComments\Client\Model\CreateHashTagBody(); // \FastComments\Client\Model\CreateHashTagBody

try {
    $result = $apiInstance->addHashTag($tenant_id, $create_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]