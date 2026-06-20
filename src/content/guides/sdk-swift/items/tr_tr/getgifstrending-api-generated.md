## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| locale | string | query | Hayır |  |
| rating | string | query | Hayır |  |
| page | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsTrendingResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getGifsTrending Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let locale = "locale_example" // String |  (isteğe bağlı)
let rating = "rating_example" // String |  (isteğe bağlı)
let page = 987 // Double |  (isteğe bağlı)

PublicAPI.getGifsTrending(tenantId: tenantId, locale: locale, rating: rating, page: page) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]