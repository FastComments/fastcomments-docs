---
Derzeit online Viewer einer Seite: Personen, deren Websocket‑Session gerade die Seite abonniert.  
Gibt **anonCount + totalCount** zurück (raumweite Abonnenten, einschließlich anonymer Viewer, die wir nicht aufzählen).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten‑URL‑Kennzeichen (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: übergebe **nextAfterName** aus der vorherigen Antwort. |
| afterUserId | string | query | Nein | Cursor‑Tiebreaker: übergebe **nextAfterUserId** aus der vorherigen Antwort. Erforderlich, wenn **afterName** gesetzt ist, damit Namens‑Tie‑Einträge nicht weggelassen werden. |

## Antwort

Gibt zurück: `PageUsersOnlineResponse`

## Beispiel

[inline-code-attrs-start title = 'getOnlineUsers Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Seiten-URL-Kennzeichen (serverseitig bereinigt).
final afterName = afterName_example; // String | Cursor: übergebe nextAfterName aus der vorherigen Antwort.
final afterUserId = afterUserId_example; // String | Cursor‑Tiebreaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namens‑Tie‑Einträge nicht weggelassen werden.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]

---