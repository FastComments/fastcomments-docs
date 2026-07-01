Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Ime | Tip | Lokacija | Potrebno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-ja strani (očistljen na strežniku). |
| afterName | string | query | No | Kursor: podajte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kursor za razreševanje zavezovanja: podajte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi. |

## Response

Vrne: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifikator URL-ja strani (očistljen na strežniku).
final afterName = afterName_example; // String | Kursor: podajte nextAfterName iz prejšnjega odgovora.
final afterUserId = afterUserId_example; // String | Kursor za razreševanje zavezovanja: podajte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]