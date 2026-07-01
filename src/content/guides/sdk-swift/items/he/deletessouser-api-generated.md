## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## תגובה

מחזיר: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteSSOUserAPIResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteSSOUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמת הקוד הבאות עדיין בתהליך בטא. לכל תקלה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let deleteComments = true // Bool |  (אופציונלי)
let commentDeleteMode = "commentDeleteMode_example" // String |  (אופציונלי)

DefaultAPI.deleteSSOUser(tenantId: tenantId, id: id, options: DefaultAPI.DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]