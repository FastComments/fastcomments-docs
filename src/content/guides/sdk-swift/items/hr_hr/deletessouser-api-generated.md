## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Odgovor

Vraća: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteSSOUserAPIResponse.swift)

## Primjer

[inline-code-attrs-start title = 'deleteSSOUser Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = true // Bool |  (optional)
let commentDeleteMode = "commentDeleteMode_example" // String |  (optional)

DefaultAPI.deleteSSOUser(tenantId: tenantId, id: id, options: DefaultAPI.DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]