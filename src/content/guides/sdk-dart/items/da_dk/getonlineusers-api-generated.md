Aktuelt online‑visere af en side: personer, hvis websocketsession er abonneret på siden lige nu.  
Returnerer anonCount + totalCount (rum‑omfattende abonnenter, inklusive anonyme visere vi ikke tæller).

## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Side‑URL‑identifikator (rengjort på serveren). |
| afterName | string | query | Nej | Markør: send nextAfterName fra den forrige respons. |
| afterUserId | string | query | Nej | Markør‑tiebreaker: send nextAfterUserId fra den forrige respons. Påkrævet når afterName er sat, så navneties ikke falder fra. |

## Response

Returnerer: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Page URL identifier (cleaned server-side).
final afterName = afterName_example; // String | Cursor: pass nextAfterName from the previous response.
final afterUserId = afterUserId_example; // String | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]