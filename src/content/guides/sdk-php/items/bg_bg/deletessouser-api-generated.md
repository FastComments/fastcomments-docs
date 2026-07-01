## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| deleteComments | boolean | query | Не |  |
| commentDeleteMode | string | query | Не |  |

## Отговор

Връща: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Пример

[inline-code-attrs-start title = 'deleteSSOUser Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигуриране на упълномощаване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Разкоментирайте по-долу, за да настроите префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Това е опционално, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$options = [
    'delete_comments' => True, // bool
    'comment_delete_mode' => 'comment_delete_mode_example', // string
];


try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---