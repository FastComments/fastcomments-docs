## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| id | string | pot | Da |  |
| deleteComments | string | poizvedba | Ne |  |
| commentDeleteMode | string | poizvedba | Ne |  |

## Odgovor

Vrača: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer deleteTenantUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršnokoli težavo, prosimo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = "deleteComments_example" // String |  (neobvezno)
let commentDeleteMode = "commentDeleteMode_example" // String |  (neobvezno)

DefaultAPI.deleteTenantUser(tenantId: tenantId, id: id, deleteComments: deleteComments, commentDeleteMode: commentDeleteMode) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]