## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|---------------|--------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |

## Response

Rückgabe: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotesForUserResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getVotesForUser Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurieren Sie die API-Schlüsselautorisierung: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie die Kommentarzeichen, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls nötig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$options = [
    'user_id' => 'user_id_example', // string
    'anon_user_id' => 'anon_user_id_example', // string
];


try {
    $result = $apiInstance->getVotesForUser($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---