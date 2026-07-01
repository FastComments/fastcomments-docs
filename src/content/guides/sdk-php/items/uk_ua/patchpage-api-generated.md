## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchPageAPIResponse.php)

## Приклад

[inline-code-attrs-start title = 'patchPage Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Налаштування авторизації API ключа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно)
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Це необов’язково, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_api_page_data = new \FastComments\Client\Model\UpdateAPIPageData(); // \FastComments\Client\Model\UpdateAPIPageData


try {
    $result = $apiInstance->patchPage($tenant_id, $id, $update_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---