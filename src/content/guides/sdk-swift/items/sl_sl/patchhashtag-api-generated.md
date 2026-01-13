## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Da |  |
| tenantId | string | query | Ne |  |

## Odgovor

Vrača: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchHashTag200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer patchHashTag'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v fazi beta. Za kakršnokoli težavo, prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (neobvezno)
let updateHashTagBody = UpdateHashTagBody(tenantId: "tenantId_example", url: "url_example", tag: "tag_example") // UpdateHashTagBody |  (neobvezno)

DefaultAPI.patchHashTag(tag: tag, tenantId: tenantId, updateHashTagBody: updateHashTagBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]