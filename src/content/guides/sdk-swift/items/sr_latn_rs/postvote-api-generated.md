## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer postVote'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let direction = "direction_example" // String |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

ModerationAPI.postVote(commentId: commentId, direction: direction, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]