## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentBanStatusResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getCommentBanStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršnokoli težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (neobvezno)

ModerationAPI.getCommentBanStatus(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]