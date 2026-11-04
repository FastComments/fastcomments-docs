Prošli komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Kursorska paginacija po commenterName: server prelazi parcijalni {tenantId, urlId, commenterName}
indeks od afterName napred pomoću $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na serveru). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razdvajanje jednakih vrednosti: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi sa istim imenom ne bi bili izgubljeni. |

## Odgovor

Vraća: `PageUsersOfflineResponse`

## Primer

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-a stranice (očišćen na serveru).
final afterName = afterName_example; // String | Kursor: prosledite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor za razdvajanje jednakih vrednosti: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi sa istim imenom ne bi bili izgubljeni.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]