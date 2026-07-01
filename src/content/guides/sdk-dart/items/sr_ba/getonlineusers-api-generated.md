Trenutno online gledatelji stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu. Vraća anonCount + totalCount (pretplatnici u cijeloj sobi, uključujući anonimne gledatelje koje ne izlistavamo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL‑a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Raspravljanje tjedera kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako se ne bi izgubili zapisi kod podudaranja imena. |

## Odgovor

Vraća: `PageUsersOnlineResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL‑a stranice (čišćen na serveru).
final afterName = afterName_example; // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Raspravljanje tjedera kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako se ne bi izgubili zapisi kod podudaranja imena.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]