## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Приклад

[inline-code-attrs-start title = 'updateEmailTemplate Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації API ключем: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використаний `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // рядок
$id = 'id_example'; // рядок
$update_email_template_body = new \FastComments\Client\Model\UpdateEmailTemplateBody(); // \FastComments\Client\Model\UpdateEmailTemplateBody

try {
    $result = $apiInstance->updateEmailTemplate($tenant_id, $id, $update_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]