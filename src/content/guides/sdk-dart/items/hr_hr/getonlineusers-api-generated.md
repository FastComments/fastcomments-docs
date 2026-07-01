Trenutni online gledatelji stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu. Vraća anonCount + totalCount (pretplatnici po cijeloj sobi, uključujući anonimne gledatelje koje ne izričito brojimo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na strani poslužitelja). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razrješavač nereda: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se ne bi izostavile stavke zbog istih imena. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-a stranice (čišćen na strani poslužitelja).
final afterName = afterName_example; // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor razrješavač nereda: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se ne bi izostavile stavke zbog istih imena.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]