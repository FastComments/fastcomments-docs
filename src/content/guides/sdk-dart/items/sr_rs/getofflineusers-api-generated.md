Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (čistite na serveru). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor tiebreaker: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako bi se izbeglo izostavljanje unosa zbog vezivanja po imenu. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL stranice (čistite na serveru).
final afterName = afterName_example; // String | Kursor: prosledite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor tiebreaker: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako bi se izbeglo izostavljanje unosa zbog vezivanja po imenu.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]