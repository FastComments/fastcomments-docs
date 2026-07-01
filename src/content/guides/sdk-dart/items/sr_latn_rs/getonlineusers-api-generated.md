---
Trenutno online posmatrači stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici u čitavoj sobi, uključujući anonimne posmatrače koje ne navodimo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čist na serveru). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razrešivač izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi izostavile stavke koje imaju isto ime. |

## Response

Vraća: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-a stranice (čist na serveru).
final afterName = afterName_example; // String | Kursor: prosledite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor razrešivač izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi izostavile stavke koje imaju isto ime.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]

---