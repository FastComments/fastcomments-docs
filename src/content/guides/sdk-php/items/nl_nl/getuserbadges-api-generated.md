## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| badgeId | string | query | Nee |  |
| type | number | query | Nee |  |
| displayedOnComments | boolean | query | Nee |  |
| limit | number | query | Nee |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadges200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'getUserBadges Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutelautorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal hieronder de commentaarteken weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$badge_id = 'badge_id_example'; // string
$type = 3.4; // float
$displayed_on_comments = True; // bool
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadges($tenant_id, $user_id, $badge_id, $type, $displayed_on_comments, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---