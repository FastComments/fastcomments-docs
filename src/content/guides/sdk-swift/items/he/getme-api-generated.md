Identifies the credential in use: the tenant it belongs to and, for OAuth tokens, the user who authorized it.  
אינטגרציות משתמשות בזה כדי לבדוק חיבור ולתייג אותו.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

מחזיר: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמת getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הקוד הבא עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
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