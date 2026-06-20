## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Nein |  |
| includeByIP | boolean | query | Nein |  |
| includeByEmailDomain | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkPreBanSummary.swift)

## Beispiel

[inline-code-attrs-start title = 'postBulkPreBanSummary Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta. Bei Problemen melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let bulkPreBanParams = BulkPreBanParams(commentIds: ["commentIds_example"]) // BulkPreBanParams | 
let includeByUserIdAndEmail = true // Bool |  (optional)
let includeByIP = true // Bool |  (optional)
let includeByEmailDomain = true // Bool |  (optional)
let sso = "sso_example" // String |  (optional)

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