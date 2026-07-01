Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore URL della pagina (pulito dal server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Risoluzione dei pareggi del cursore: passa nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato affinché i pareggi di nome non eliminino voci. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Esempio getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificatore URL della pagina (pulito dal server).
final afterName = afterName_example; // String | Cursore: passa nextAfterName dalla risposta precedente.
final afterUserId = afterUserId_example; // String | Risoluzione dei pareggi del cursore: passa nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato affinché i pareggi di nome non eliminino voci.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]