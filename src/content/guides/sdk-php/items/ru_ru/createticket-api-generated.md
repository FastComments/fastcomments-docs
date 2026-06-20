## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Да |  |

## Ответ

Возвращает: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTicketResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример createTicket'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$create_ticket_body = new \FastComments\Client\Model\CreateTicketBody(); // \FastComments\Client\Model\CreateTicketBody

try {
    $result = $apiInstance->createTicket($tenant_id, $user_id, $create_ticket_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTicket: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]