Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a “Members” section.  
Cursor pagination on commenterName: server walks the partial `{tenantId, urlId, commenterName}` index from afterName forward via `$gt`, no `$skip` cost.

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Bezeichner (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | No | Cursor‑Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namensbindungen keine Einträge verlieren. |

## Antwort

Rückgabe: `PageUsersOfflineResponse`

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Seiten-URL-Bezeichner (serverseitig bereinigt).
final afterName = afterName_example; // String | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort.
final afterUserId = afterUserId_example; // String | Cursor‑Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Namensbindungen keine Einträge verlieren.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]