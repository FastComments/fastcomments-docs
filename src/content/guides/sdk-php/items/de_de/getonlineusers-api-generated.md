Derzeit online angezeigte Betrachter einer Seite: Personen, deren Websocket‑Sitzung gerade die Seite abonniert hat.  
Gibt anonCount + totalCount zurück (räumliche Abonnenten, einschließlich anonymer Betrachter, die wir nicht auflisten).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|------|--------------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten‑URL‑Identifikator (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | Nein | Cursor‑Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Seiten-URL-Identifikator (serverseitig bereinigt).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort.
    'after_user_id' => 'after_user_id_example', // string | Cursor‑Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---