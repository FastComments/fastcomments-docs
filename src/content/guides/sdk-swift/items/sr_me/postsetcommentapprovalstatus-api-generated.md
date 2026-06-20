---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | putanja | Da |  |
| approved | boolean | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Primjer

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let approved = true // Bool |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

ModerationAPI.postSetCommentApprovalStatus(commentId: commentId, approved: approved, sso: sso) { (response, error) in
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