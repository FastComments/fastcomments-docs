Trenutno spletni uporabniki strani: ljudje, katerih seansa websocket je trenutno naročena na stran right now.  
Vrne anonCount + totalCount (naročniki po celotni sobi, vključno z anonimnimi gledalci, ki jih ne štejemo).

## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-ja strani (čiščen na strežniku). |
| afterName | string | query | No | Kazalec: pošlji nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kazalec za razreševanje neodločnosti: pošlji nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se z imenskimi neodločnostmi ne izpustijo vnosi. |

## Response

Vrne: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-ja strani (čiščen na strežniku).
final afterName = afterName_example; // String | Kazalec: pošlji nextAfterName iz prejšnjega odgovora.
final afterUserId = afterUserId_example; // String | Kazalec za razreševanje neodločnosti: pošlji nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je nastavljen afterName, da se z imenskimi neodločnostmi ne izpustijo vnosi.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]