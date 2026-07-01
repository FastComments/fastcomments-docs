Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parametreler

| Name | Type | Location | Required | Açıklama |
|------|------|----------|----------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL kimliği (sunucu tarafında temizlenmiş). |
| afterName | string | query | No | İmleç: bir önceki yanıttan nextAfterName değerini geç. |
| afterUserId | string | query | No | İmleç bağlaç: bir önceki yanıttan nextAfterUserId değerini geç. afterName ayarlandığında, isim eşitlikleri girişleri düşürmesin diye gereklidir. |

## Yanıt

Döndürür: `PageUsersOfflineResponse`

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Sayfa URL kimliği (sunucu tarafında temizlenmiş).
final afterName = afterName_example; // String | İmleç: bir önceki yanıttan nextAfterName değerini geç.
final afterUserId = afterUserId_example; // String | İmleç bağlaç: bir önceki yanıttan nextAfterUserId değerini geç. afterName ayarlandığında, isim eşitlikleri girişleri düşürmesin diye gereklidir.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]