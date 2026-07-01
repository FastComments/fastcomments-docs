Past komentatori na stranici koji trenutno NISU online. Sortirani po displayName.  
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije „Members“.  
Kursor paginacija po commenterName: server prolazi kroz djelimični {tenantId, urlId, commenterName} indeks od afterName naprijed koristeći $gt, bez $skip troška.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL‑a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razdjelnik: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se izbjeglo izostavljanje unosa zbog podudaranja imena. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | Identifikator URL‑a stranice (čišćen na serveru).
final afterName = afterName_example; // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor razdjelnik: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se izbjeglo izostavljanje unosa zbog podudaranja imena.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]