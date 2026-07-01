Past komentatori na stranici koji NIJE trenutno online. Sortirano po **displayName**.  
Koristite ovo nakon što iskoristite `/users/online` za prikaz sekcije „Članovi“.  
Kursor paginacija po **commenterName**: server prolazi kroz djelomični `{tenantId, urlId, commenterName}` indeks od **afterName** naprijed putem `$gt`, bez troška `$skip`.

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da | |
| urlId | string | query | Da | Identifikator URL stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: proslijedite **nextAfterName** iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor razrješivač: proslijedite **nextAfterUserId** iz prethodnog odgovora. Potrebno kada je **afterName** postavljen da se ne izgube zapisi pri istoimenim imenima. |

## Odgovor

Vraća: `PageUsersOfflineResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL stranice (čišćen na serveru).
final afterName = afterName_example; // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor razrješivač: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen da se ne izgube zapisi pri istoimenim imenima.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]