## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vrača: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserManualBadgesResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getManualBadgesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za morebitne težave poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgesUserId = "badgesUserId_example" // String |  (izbirno)
let commentId = "commentId_example" // String |  (izbirno)
let sso = "sso_example" // String |  (izbirno)

ModerationAPI.getManualBadgesForUser(badgesUserId: badgesUserId, commentId: commentId, sso: sso) { (response, error) in
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