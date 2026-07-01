Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.  
Brug dette efter at have udtømt /users/online for at gengive en "Members" sektion.  
Cursor-paginering på commenterName: serveren gennemgår den delvise {tenantId, urlId, commenterName}  
indeks fra afterName fremad via $gt, ingen $skip omkostning.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (renset på serveren). |
| afterName | string | query | No | Cursor: send nextAfterName fra det foregående svar. |
| afterUserId | string | query | No | Cursor-tiebreaker: send nextAfterUserId fra det foregående svar. Krævet når afterName er angivet, så navne-ties ikke udelader poster. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Side-URL-identifikator (renset på serveren).
final afterName = afterName_example; // String | Cursor: send nextAfterName fra det foregående svar.
final afterUserId = afterUserId_example; // String | Cursor-tiebreaker: send nextAfterUserId fra det foregående svar. Krævet når afterName er angivet, så navne-ties ikke udelader poster.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]