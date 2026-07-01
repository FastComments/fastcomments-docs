## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|--------------|
| tenantId | string | query | Ja |  |

## Antwort

Rückgabe: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APICreateUserBadgeResponse.php)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für createUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Konfigurieren Sie die API-Schlüsselautorisierung: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Kommentieren Sie die folgende Zeile aus, um ein Präfix festzulegen (z. B. Bearer) für den API-Schlüssel, falls erforderlich


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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