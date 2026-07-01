## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |

## Відповідь

Повертає: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад flagComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію за допомогою API ключа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // рядок
$id = 'id_example'; // рядок
$options = [
    'user_id' => 'user_id_example', // рядок
    'anon_user_id' => 'anon_user_id_example', // рядок
];


try {
    $result = $apiInstance->flagComment($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->flagComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]