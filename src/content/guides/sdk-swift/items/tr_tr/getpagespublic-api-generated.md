Bir tenant için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenmiş özel yapılandırmada `enableFChat`'in true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | No | 1..200, varsayılan 50 |
| q | string | query | No | İsteğe bağlı, büyük-küçük harfe duyarsız başlık önek filtresi. |
| sortBy | string | query | No | Sıralama düzeni. `updatedAt` (varsayılan, en yeni ilk), `commentCount` (en çok yorumlu ilk), veya `title` (alfabetik). |
| hasComments | boolean | query | No | Eğer true ise, yalnızca en az bir yoruma sahip sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir. (isteğe bağlı)
let limit = 987 // Int | 1..200, varsayılan 50 (isteğe bağlı)
let q = "q_example" // String | İsteğe bağlı, büyük-küçük harfe duyarsız başlık önek filtresi. (isteğe bağlı)
let sortBy = PagesSortBy() // PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni ilk), `commentCount` (en çok yorumlu ilk), veya `title` (alfabetik). (isteğe bağlı)
let hasComments = true // Bool | Eğer true ise, yalnızca en az bir yoruma sahip sayfaları döndürür. (isteğe bağlı)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---