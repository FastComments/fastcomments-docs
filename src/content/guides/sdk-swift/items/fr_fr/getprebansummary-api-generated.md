## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| includeByUserIdAndEmail | boolean | query | Non |  |
| includeByIP | boolean | query | Non |  |
| includeByEmailDomain | boolean | query | Non |  |
| sso | string | query | Non |  |

## Response

Retourne : [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## Exemple

[inline-code-attrs-start title = 'getPreBanSummary Exemple'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les échantillons de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool | (optionnel)
let includeByIP = true // Bool | (optionnel)
let includeByEmailDomain = true // Bool | (optionnel)
let sso = "sso_example" // String | (optionnel)

ModerationAPI.getPreBanSummary(tenantId: tenantId, commentId: commentId, options: ModerationAPI.GetPreBanSummaryOptions(includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]