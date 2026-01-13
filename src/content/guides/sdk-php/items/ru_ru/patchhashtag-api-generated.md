## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Yes |  |
| tenantId | string | query | No |  |

## Ответ

Возвращает: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchHashTag200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример patchHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Настройка авторизации по API ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Раскомментируйте ниже, чтобы установить префикс (например Bearer) для API ключа, если это необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // строка
$tenant_id = 'tenant_id_example'; // строка
$update_hash_tag_body = new \FastComments\Client\Model\UpdateHashTagBody(); // \FastComments\Client\Model\UpdateHashTagBody

try {
    $result = $apiInstance->patchHashTag($tag, $tenant_id, $update_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]