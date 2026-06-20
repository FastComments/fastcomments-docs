---
Frühere Kommentatoren auf der Seite, die derzeit NICHT online sind. Nach displayName sortiert.
Verwenden Sie dies, nachdem Sie /users/online erschöpft haben, um einen "Mitglieder"-Abschnitt darzustellen.
Cursor-Paginierung auf commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index von afterName vorwärts über $gt, keine $skip-Kosten.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Response

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
$after_name = 'after_name_example'; // string | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort.
$after_user_id = 'after_user_id_example'; // string | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge verloren gehen.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---