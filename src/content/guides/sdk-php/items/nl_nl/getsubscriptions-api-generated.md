## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |

## Respons

Retourneert: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSubscriptionsAPIResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getSubscriptions Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutelauthorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Verwijder de commentaartekens hieronder om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als je een aangepaste HTTP-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string


try {
    $result = $apiInstance->getSubscriptions($tenant_id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSubscriptions: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]