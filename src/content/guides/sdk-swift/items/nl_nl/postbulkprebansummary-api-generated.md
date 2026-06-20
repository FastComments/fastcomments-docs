## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Nee |  |
| includeByIP | boolean | query | Nee |  |
| includeByEmailDomain | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkPreBanSummary.swift)

## Voorbeeld

[inline-code-attrs-start title = 'postBulkPreBanSummary Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in beta. Bij problemen, meld deze via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let bulkPreBanParams = BulkPreBanParams(commentIds: ["commentIds_example"]) // BulkPreBanParams | 
let includeByUserIdAndEmail = true // Bool |  (optioneel)
let includeByIP = true // Bool |  (optioneel)
let includeByEmailDomain = true // Bool |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

ModerationAPI.postBulkPreBanSummary(bulkPreBanParams: bulkPreBanParams, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]