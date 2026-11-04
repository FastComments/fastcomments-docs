Frühere Kommentatoren auf der Seite, die NICHT aktuell online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem /users/online erschöpft ist, um einen „Mitglieder"-Bereich darzustellen.
Cursor-Paginierung über commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts über $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Identifikator (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: nextAfterName aus der vorherigen Antwort übergeben. |
| afterUserId | string | query | Nein | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren. |

## Antwort

Gibt zurück: `PageUsersOfflineResponse`

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Seiten-URL-Identifikator (serverseitig bereinigt).
final afterName = afterName_example; // String | Cursor: nextAfterName aus der vorherigen Antwort übergeben.
final afterUserId = afterUserId_example; // String | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge verlieren.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]