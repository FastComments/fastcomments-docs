## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| includeEmail | boolean | query | Nee |  |
| includeIP | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICommentResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getModerationComment Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog beta. Voor eventuele problemen, rapporteer via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let includeEmail = true // Bool |  (optional)
let includeIP = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

ModerationAPI.getModerationComment(tenantId: tenantId, commentId: commentId, options: ModerationAPI.GetModerationCommentOptions(includeEmail: includeEmail, includeIP: includeIP, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]