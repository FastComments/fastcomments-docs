## Parameters

| Naam | Type | Location | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| includeEmail | boolean | query | Nee |  |
| includeIP | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getModerationComment Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (optioneel)
let includeIP = true // Bool |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

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