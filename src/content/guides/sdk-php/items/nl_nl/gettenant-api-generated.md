## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Response

Geeft terug: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenant200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'getTenant Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer autorisatie met API-sleutel: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal de commentaarstrepen hieronder weg om een voorvoegsel in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client op die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getTenant($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenant: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---