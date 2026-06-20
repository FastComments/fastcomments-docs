## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| includeEmail | boolean | query | Nie |  |
| includeIP | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getModerationComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu wciąż są w wersji beta. W razie problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (opcjonalne)
let includeIP = true // Bool |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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

---