## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetLogsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (opcjonalne)

ModerationAPI.getLogs(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]