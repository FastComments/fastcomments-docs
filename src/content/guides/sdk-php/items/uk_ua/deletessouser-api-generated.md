## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| deleteComments | boolean | query | Ні |  |
| commentDeleteMode | string | query | Ні |  |

## Відповідь

Повертає: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteSSOUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію за допомогою API-ключа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$delete_comments = True; // bool
$comment_delete_mode = 'comment_delete_mode_example'; // string

try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $delete_comments, $comment_delete_mode);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]