Sayfada şu anda çevrimiçi OLMAYAN, daha önce yorum yapmış kullanıcılar. displayName'e göre sıralı.
Bir "Üyeler" bölümü oluşturmak için /users/online tükendikten sonra bunu kullanın.
commenterName üzerinde imleç sayfalama: sunucu, {tenantId, urlId, commenterName} kısmi indeksini
afterName'den $gt ile ileriye doğru tarar, $skip maliyeti yoktur.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName değerini iletin. |
| afterUserId | string | query | Hayır | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini iletin. afterName ayarlandığında zorunludur, böylece aynı isimli girdiler düşmez. |

## Yanıt

Döndürür: `PageUsersOfflineResponse`

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
final afterName = afterName_example; // String | İmleç: önceki yanıttan nextAfterName değerini iletin.
final afterUserId = afterUserId_example; // String | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini iletin. afterName ayarlandığında zorunludur, böylece aynı isimli girdiler düşmez.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]