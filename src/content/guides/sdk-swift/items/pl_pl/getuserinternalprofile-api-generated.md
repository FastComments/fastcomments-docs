## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserInternalProfile'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu, proszę zgłosić go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
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