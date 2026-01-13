## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchPageAPIResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'patchPage Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPIPageData = UpdateAPIPageData(isClosed: false, accessibleByGroupIds: ["accessibleByGroupIds_example"], title: "title_example", url: "url_example", urlId: "urlId_example") // UpdateAPIPageData | 

DefaultAPI.patchPage(tenantId: tenantId, id: id, updateAPIPageData: updateAPIPageData) { (response, error) in
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