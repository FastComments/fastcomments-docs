## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| urlId | string | query | Nein |  |
| fromCommentId | string | query | Nein |  |
| viewed | boolean | query | Nein |  |
| type | string | query | Nein |  |

## Response

Gibt zurück: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationCountResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getNotificationCount Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Authentifizierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie die Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu konfigurieren, falls erforderlich
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen eigenen HTTP-Client verwenden möchten, übergeben Sie einen Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional; `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$url_id = 'url_id_example'; // string
$from_comment_id = 'from_comment_id_example'; // string
$viewed = True; // bool
$type = 'type_example'; // string

try {
    $result = $apiInstance->getNotificationCount($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]