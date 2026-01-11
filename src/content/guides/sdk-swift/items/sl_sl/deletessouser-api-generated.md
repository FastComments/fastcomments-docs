## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| deleteComments | boolean | query | Ne |  |
| commentDeleteMode | string | query | Ne |  |

## Odziv

Vrača: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteSSOUserAPIResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer deleteSSOUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršnokoli težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = true // Bool |  (neobvezno)
let commentDeleteMode = "commentDeleteMode_example" // String |  (neobvezno)

DefaultAPI.deleteSSOUser(tenantId: tenantId, id: id, deleteComments: deleteComments, commentDeleteMode: commentDeleteMode) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]