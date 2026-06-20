## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| value | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSiteSearchResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getSearchSites Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

ModerationAPI.getSearchSites(value: value, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]