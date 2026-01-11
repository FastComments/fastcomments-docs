## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrača: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddPageAPIResponse.swift)

## Primer

[inline-code-attrs-start title = 'addPage Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Če naletite na težavo, jo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPIPageData = CreateAPIPageData(accessibleByGroupIds: ["accessibleByGroupIds_example"], rootCommentCount: 123, commentCount: 123, title: "title_example", url: "url_example", urlId: "urlId_example") // CreateAPIPageData | 

DefaultAPI.addPage(tenantId: tenantId, createAPIPageData: createAPIPageData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]