## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTicketResponse.php)

## Örnek

[inline-code-attrs-start title = 'createTicket Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtar yetkilendirmesini yapılandırın: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için öneki (ör. Bearer) ayarlamak için aşağıdaki satırın başındaki yorumu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcusu kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır; varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
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