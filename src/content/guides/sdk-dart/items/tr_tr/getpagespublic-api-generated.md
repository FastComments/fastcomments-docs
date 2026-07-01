List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Opak sayfalama imleci `nextCursor` olarak önceki bir istekte döndürüldü. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı büyük/küçük harf duyarsız başlık önek filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | Eğer true ise, yalnızca en az bir yorumu olan sayfaları döndür. |

## Response

Returns: `GetPublicPagesResponse`

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Opak sayfalama imleci `nextCursor` olarak önceki bir istekte döndürüldü. Aynı `sortBy` ile ilişkilidir.
final limit = 56; // int | 1..200, varsayılan 50
final q = q_example; // String | İsteğe bağlı büyük/küçük harf duyarsız başlık önek filtresi.
final sortBy = ; // PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik).
final hasComments = true; // bool | Eğer true ise, yalnızca en az bir yorumu olan sayfaları döndür.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]