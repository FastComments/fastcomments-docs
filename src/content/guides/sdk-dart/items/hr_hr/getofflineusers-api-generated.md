Prošli komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Cursor paginacija po commenterName: server prolazi parcijalni {tenantId, urlId, commenterName}
indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na serveru). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrješenje istih imena: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi s istim imenom ne bi bili izgubljeni. |

## Odgovor

Vraća: `PageUsersOfflineResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer za getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-a stranice (očišćen na serveru).
final afterName = afterName_example; // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
final afterUserId = afterUserId_example; // String | Kursor za razrješenje istih imena: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi s istim imenom ne bi bili izgubljeni.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]