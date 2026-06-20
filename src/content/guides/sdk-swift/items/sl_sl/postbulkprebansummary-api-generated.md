## Parametri

| Name | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Ne |  |
| includeByIP | boolean | query | Ne |  |
| includeByEmailDomain | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkPreBanSummary.swift)

## Primer

[inline-code-attrs-start title = 'Primer postBulkPreBanSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za morebitne težave, prosimo prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let bulkPreBanParams = BulkPreBanParams(commentIds: ["commentIds_example"]) // BulkPreBanParams | 
let includeByUserIdAndEmail = true // Bool |  (izbirno)
let includeByIP = true // Bool |  (izbirno)
let includeByEmailDomain = true // Bool |  (izbirno)
let sso = "sso_example" // String |  (izbirno)

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