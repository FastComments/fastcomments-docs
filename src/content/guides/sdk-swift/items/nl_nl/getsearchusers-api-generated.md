---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| value | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationUserSearchResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getSearchUsers Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

ModerationAPI.getSearchUsers(value: value, sso: sso) { (response, error) in
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