## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APICreateUserBadgeResponse.php)

## Beispiel

[inline-code-attrs-start title = 'createUserBadge Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Key-Autorisierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie den Kommentar unten, um ein Präfix (z.B. Bearer) für den API-Key zu setzen, falls nötig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen eigenen HTTP-Client verwenden möchten, übergeben Sie einen Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional; `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_user_badge_params = new \FastComments\Client\Model\CreateUserBadgeParams(); // \FastComments\Client\Model\CreateUserBadgeParams

try {
    $result = $apiInstance->createUserBadge($tenant_id, $create_user_badge_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]