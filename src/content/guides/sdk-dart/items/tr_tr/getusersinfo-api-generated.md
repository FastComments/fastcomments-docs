Bir kiracı (tenant) için toplu kullanıcı bilgisi. Verilen userIds değerlerine göre, User / SSOUser kaynaklarından görüntüleme bilgilerini döndürür.
Yorum widget'ı tarafından, bir varlık (presence) olayı aracılığıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yoktur: gizlilik tek tip şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| ids | string | query | Evet | Virgülle ayrılmış userIds değerleri. |

## Yanıt

Döndürür: `PageUsersInfoResponse`

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Virgülle ayrılmış userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]