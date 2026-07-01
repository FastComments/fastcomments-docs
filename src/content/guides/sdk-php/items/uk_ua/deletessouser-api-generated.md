## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| deleteComments | boolean | query | Ні |  |
| commentDeleteMode | string | query | Ні |  |

## Відповідь

Returns: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Приклад

[inline-code-attrs-start title = 'deleteSSOUser Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, за потреби
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // рядок
$id = 'id_example'; // рядок
$options = [
    'delete_comments' => True, // булевий
    'comment_delete_mode' => 'comment_delete_mode_example', // рядок
];


try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]