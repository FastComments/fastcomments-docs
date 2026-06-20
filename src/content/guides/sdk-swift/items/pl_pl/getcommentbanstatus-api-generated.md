## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentBanStatusResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentBanStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku problemu prosimy zgłaszać go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (opcjonalne)

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

---