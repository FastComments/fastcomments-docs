Aktuell online befindliche Zuschauer einer Seite: Personen, deren WebSocket-Sitzung gerade auf die Seite abonniert ist. Gibt anonCount + totalCount zurück (raumweit abonnierte Nutzer, einschließlich anonymer Zuschauer, die wir nicht einzeln aufzählen).

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Kennung (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | No | Cursor-Tie-Breaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen. |

## Antwort

Gibt zurück: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getOnlineUsers Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Seiten-URL-Kennung (serverseitig bereinigt).
$after_name = 'after_name_example'; // string | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort.
$after_user_id = 'after_user_id_example'; // string | Cursor-Tie-Breaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---