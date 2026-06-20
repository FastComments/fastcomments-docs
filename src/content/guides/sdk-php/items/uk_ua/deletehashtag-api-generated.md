## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Так |  |
| tenantId | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використовуватися `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // рядок
$tenant_id = 'tenant_id_example'; // рядок
$delete_hash_tag_request_body = new \FastComments\Client\Model\DeleteHashTagRequestBody(); // \FastComments\Client\Model\DeleteHashTagRequestBody

try {
    $result = $apiInstance->deleteHashTag($tag, $tenant_id, $delete_hash_tag_request_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]