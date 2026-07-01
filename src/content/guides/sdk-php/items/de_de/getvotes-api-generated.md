## Parameter

| Name     | Typ    | Ort   | Erforderlich | Beschreibung |
|----------|--------|-------|--------------|--------------|
| tenantId | string | query | Ja           |  |
| urlId    | string | query | Ja           |  |

## Antwort

Rückgabe: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotesResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getVotes Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Authentifizierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entkommentiere die nachfolgende Zeile, um ein Präfix (z.B. Bearer) für den API-Schlüssel festzulegen, falls nötig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn du einen benutzerdefinierten HTTP-Client verwenden möchtest, übergebe deinen Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string


try {
    $result = $apiInstance->getVotes($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]