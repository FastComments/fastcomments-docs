Pretekli komentatorji na strani, ki trenutno NISO na spletu. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online za prikaz razdelka "Members".
Straniščna paginacija po commenterName: strežnik prehaja delni indeks {tenantId, urlId, commenterName}
od afterName naprej prek $gt, brez stroškov $skip.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-ja strani (očiščen na strežniku). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kazalni odločilni kriterij: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezen, ko je nastavljen afterName, da vnosi z istim imenom ne izginejo. |

## Odgovor

Vrne: `PageUsersOfflineResponse`

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-ja strani (očiščen na strežniku).
final afterName = afterName_example; // String | Kazalec: posredujte nextAfterName iz prejšnjega odgovora.
final afterUserId = afterUserId_example; // String | Kazalni odločilni kriterij: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezen, ko je nastavljen afterName, da vnosi z istim imenom ne izginejo.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]