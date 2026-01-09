## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nein |  |

## Antwort

Gibt zurück: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSubscriptionAPIResponse.php)

## Beispiel

[inline-code-attrs-start title = 'deleteSubscription Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Authentifizierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie die Kommentarzeichen unten, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->deleteSubscription($tenant_id, $id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]