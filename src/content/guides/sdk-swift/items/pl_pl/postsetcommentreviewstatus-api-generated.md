## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Tak |  |
| commentId | string | path | Tak |  |
| reviewed | boolean | query | Nie |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Przykład

[inline-code-attrs-start title = 'postSetCommentReviewStatus Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące próbki kodu są wciąż w wersji beta. W przypadku problemów proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let commentId = "commentId_example" // String |
let reviewed = true // Bool | (opcjonalnie)
let broadcastId = "broadcastId_example" // String | (opcjonalnie)
let sso = "sso_example" // String | (opcjonalnie)

ModerationAPI.postSetCommentReviewStatus(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostSetCommentReviewStatusOptions(reviewed: reviewed, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]