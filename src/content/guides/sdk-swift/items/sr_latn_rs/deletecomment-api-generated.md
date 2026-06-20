## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| contextUserId | string | query | Ne |  |
| isLive | boolean | query | Ne |  |

## Odgovor

Vraća: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentResult.swift)

## Primer

[inline-code-attrs-start title = 'deleteComment Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite ga preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let contextUserId = "contextUserId_example" // String |  (opciono)
let isLive = true // Bool |  (opciono)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, contextUserId: contextUserId, isLive: isLive) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]