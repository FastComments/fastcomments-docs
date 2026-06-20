## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkPreBanSummary.swift)

## דוגמה

[inline-code-attrs-start title = 'postBulkPreBanSummary דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמות הקוד הבאות עדיין בגרסת בטא. אם ישנה בעיה, יש לדווח בכתובת http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let bulkPreBanParams = BulkPreBanParams(commentIds: ["commentIds_example"]) // BulkPreBanParams | 
let includeByUserIdAndEmail = true // Bool |  (אופציונלי)
let includeByIP = true // Bool |  (אופציונלי)
let includeByEmailDomain = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

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

---