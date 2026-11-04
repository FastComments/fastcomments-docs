Bir kiracı (tenant) için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.
SSO gerektiren sayfalar, isteği yapan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorumlu önce) veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | true ise, yalnızca en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: `GetPublicPagesResponse`

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır.
final limit = 56; // int | 1..200, varsayılan 50
final q = q_example; // String | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi.
final sortBy = ; // PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorumlu önce) veya `title` (alfabetik).
final hasComments = true; // bool | true ise, yalnızca en az bir yorumu olan sayfaları döndürür.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]