Frühere Kommentatoren auf der Seite, die aktuell NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem /users/online ausgeschöpft ist, um einen „Mitglieder"-Bereich darzustellen.
Cursor-Paginierung über commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts mittels $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Bezeichner (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: nextAfterName aus der vorherigen Antwort übergeben. |
| afterUserId | string | query | Nein | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Seiten-URL-Bezeichner (serverseitig bereinigt).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: nextAfterName aus der vorherigen Antwort übergeben.
    'after_user_id' => 'after_user_id_example', // string | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]