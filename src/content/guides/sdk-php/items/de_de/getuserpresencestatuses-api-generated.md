## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlIdWS | string | query | Ja |  |
| userIds | string | query | Ja |  |

## Antwort

Gibt zurück: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatusesResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getUserPresenceStatuses Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id_ws = 'url_id_ws_example'; // string
$user_ids = 'user_ids_example'; // string

try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]