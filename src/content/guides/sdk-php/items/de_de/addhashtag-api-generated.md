## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nein |  |

## Antwort

Gibt zurück: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateHashTagResponse.php)

## Beispiel

[inline-code-attrs-start title = 'addHashTag Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurieren der API-Schlüssel-Authentifizierung: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Unten auskommentieren, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, standardmäßig wird `GuzzleHttp\Client` verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_hash_tag_body = new \FastComments\Client\Model\CreateHashTagBody(); // \FastComments\Client\Model\CreateHashTagBody

try {
    $result = $apiInstance->addHashTag($tenant_id, $create_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]