## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchPageAPIResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад patchPage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації API-ключем: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, що реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // рядок
$id = 'id_example'; // рядок
$update_api_page_data = new \FastComments\Client\Model\UpdateAPIPageData(); // \FastComments\Client\Model\UpdateAPIPageData

try {
    $result = $apiInstance->patchPage($tenant_id, $id, $update_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]