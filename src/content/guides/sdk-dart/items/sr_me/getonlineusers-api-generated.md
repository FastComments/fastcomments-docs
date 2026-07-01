Trenutno online gledatelji stranice: ljudi čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici na cijelu sobu, uključujući anonimne gledatelje koje ne nabrajamo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL‑a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razdvajač: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen da se ne izgube unosi zbog istog imena. |

## Odgovor

Vraća: `PageUsersOnlineResponse`

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL‑a stranice (čišćen na serveru).
final afterName = afterName_example; // String | Kursor: pošaljite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor razdvajač: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen da se ne izgube unosi zbog istog imena.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]