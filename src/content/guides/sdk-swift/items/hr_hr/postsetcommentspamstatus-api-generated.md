## Parametri

| Naziv | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| spam | boolean | query | Ne |  |
| permNotSpam | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Primjer

[inline-code-attrs-start title = 'postSetCommentSpamStatus Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let spam = true // Bool |  (neobavezno)
let permNotSpam = true // Bool |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

ModerationAPI.postSetCommentSpamStatus(commentId: commentId, spam: spam, permNotSpam: permNotSpam, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]