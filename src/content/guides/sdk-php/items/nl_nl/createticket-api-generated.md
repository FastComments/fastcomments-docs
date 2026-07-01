## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Ja |  |

## Respons

Returns: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTicketResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'createTicket Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutautorisatie: api_key
// Verwijder de commentaar op de onderstaande regel om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als je een aangepaste HTTP-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
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