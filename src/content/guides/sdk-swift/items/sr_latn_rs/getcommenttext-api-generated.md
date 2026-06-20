## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| editKey | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIGetCommentTextResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getCommentText'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let editKey = "editKey_example" // String |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

PublicAPI.getCommentText(tenantId: tenantId, commentId: commentId, editKey: editKey, sso: sso) { (response, error) in
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