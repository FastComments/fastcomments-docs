## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| text-search | string | query | Hayır |  |
| byIPFromComment | string | query | Hayır |  |
| filter | string | query | Hayır |  |
| searchFilters | string | query | Hayır |  |
| demo | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getCount Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta sürümündedir. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let textSearch = "textSearch_example" // String |  (opsiyonel)
let byIPFromComment = "byIPFromComment_example" // String |  (opsiyonel)
let filter = "filter_example" // String |  (opsiyonel)
let searchFilters = "searchFilters_example" // String |  (opsiyonel)
let demo = true // Bool |  (opsiyonel)
let sso = "sso_example" // String |  (opsiyonel)

ModerationAPI.getCount(tenantId: tenantId, options: ModerationAPI.GetCountOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]