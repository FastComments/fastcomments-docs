Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Bir kiracı için toplu kullanıcı bilgileri. Verilen userId'ler, User / SSOUser'dan görüntüleme bilgilerini döndürür.

Used by the comment widget to enrich users that just appeared via a presence event.  
Yorum widget'ı tarafından, bir varlık olayıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.

No page context: privacy is enforced uniformly (private profiles are masked).  
Sayfa bağlamı yok: gizlilik tutarlı bir şekilde uygulanır (özel profiller maskelenir).

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Virgülle ayrılmış kullanıcı kimlikleri. |

## Response

Returns: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Virgülle ayrılmış kullanıcı kimlikleri.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]