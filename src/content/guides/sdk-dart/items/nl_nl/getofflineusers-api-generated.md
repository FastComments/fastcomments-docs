Past commenters op de pagina die NIET momenteel online zijn. Gesorteerd op weergeefnaam.  
Gebruik dit na het uitputten van /users/online om een “Leden”-sectie weer te geven.  
Cursor-paginering op commenterName: de server loopt door het gedeeltelijke {tenantId, urlId, commenterName}  
index vanaf afterName vooruit via $gt, zonder $skip-kost.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificator (server‑side opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName van de vorige respons door. |
| afterUserId | string | query | Nee | Cursor‑tiebreaker: geef nextAfterUserId van de vorige respons door. Vereist wanneer afterName is ingesteld zodat naam‑ties geen items laten vallen. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Pagina-URL-identificator (server‑side opgeschoond).
final afterName = afterName_example; // String | Cursor: geef nextAfterName van de vorige respons door.
final afterUserId = afterUserId_example; // String | Cursor‑tiebreaker: geef nextAfterUserId van de vorige respons door. Vereist wanneer afterName is ingesteld zodat naam‑ties geen items laten vallen.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]