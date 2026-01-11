## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserBadge200Response.php)

## Beispiel

[inline-code-attrs-start title = 'updateUserBadge Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Autorisierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls benötigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie einen Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_user_badge_params = new \FastComments\Client\Model\UpdateUserBadgeParams(); // \FastComments\Client\Model\UpdateUserBadgeParams

try {
    $result = $apiInstance->updateUserBadge($tenant_id, $id, $update_user_badge_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]