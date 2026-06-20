## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getUserInternalProfile'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v fazi beta. Če naletite na težavo, prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

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