## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример deletePendingWebhookEvent'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Настроить авторизацию API-ключа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->deletePendingWebhookEvent($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deletePendingWebhookEvent: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---