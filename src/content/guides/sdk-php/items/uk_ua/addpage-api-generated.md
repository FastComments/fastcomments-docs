## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Відповідь

Повертає: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddPageAPIResponse.php)

## Приклад

[inline-code-attrs-start title = 'addPage Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це опціонально, `GuzzleHttp\Client` буде використано за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // рядок
$create_api_page_data = new \FastComments\Client\Model\CreateAPIPageData(); // \FastComments\Client\Model\CreateAPIPageData


try {
    $result = $apiInstance->addPage($tenant_id, $create_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---