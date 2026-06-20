## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| text-search | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSuggestResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getSearchSuggest Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen şu adres üzerinden bildirin: http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

ModerationAPI.getSearchSuggest(textSearch: textSearch, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]