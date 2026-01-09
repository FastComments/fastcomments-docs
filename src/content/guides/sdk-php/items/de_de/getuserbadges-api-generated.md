## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| badgeId | string | query | Nein |  |
| type | number | query | Nein |  |
| displayedOnComments | boolean | query | Nein |  |
| limit | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadges200Response.php)

## Beispiel

[inline-code-attrs-start title = 'getUserBadges Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Autorisierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entkommentieren Sie das Folgende, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional; `GuzzleHttp\Client` wird standardmäßig verwendet.
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