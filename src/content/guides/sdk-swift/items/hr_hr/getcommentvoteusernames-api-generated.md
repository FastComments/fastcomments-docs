## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| dir | integer | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentVoteUserNames200Response.swift)

## Primjer

[inline-code-attrs-start title = 'getCommentVoteUserNames Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta verziji. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let dir = 987 // Int | 
let sso = "sso_example" // String |  (neobavezno)

PublicAPI.getCommentVoteUserNames(tenantId: tenantId, commentId: commentId, dir: dir, sso: sso) { (response, error) in
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