Bir kiracı (tenant) için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum olan önce) veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | true ise, yalnızca en az bir yorum içeren sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır. (optional)
let limit = 987 // Int | 1..200, varsayılan 50 (optional)
let q = "q_example" // String | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum olan önce) veya `title` (alfabetik). (optional)
let hasComments = true // Bool | true ise, yalnızca en az bir yorum içeren sayfaları döndürür. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]