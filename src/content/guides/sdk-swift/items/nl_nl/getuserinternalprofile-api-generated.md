## Parameters

| Naam | Type | Locatie | Verplicht | Omschrijving |
|------|------|----------|----------|-------------|
| commentId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van getUserInternalProfile'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor problemen, meld deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

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