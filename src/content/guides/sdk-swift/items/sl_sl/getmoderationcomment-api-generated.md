## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| includeEmail | boolean | query | Ne |  |
| includeIP | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getModerationComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršnekoli težave, prosimo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (neobvezno)
let includeIP = true // Bool |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

ModerationAPI.getModerationComment(commentId: commentId, includeEmail: includeEmail, includeIP: includeIP, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]