מי הצביע על איזו אפשרות. סקרים אנונימיים אף פעם לא משיבים, סקרים המיועדים למנהלים בלבד משיבים רק למנהל או למנחה.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| page | integer | query | No |  |
| search | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'getPollVoters דוגמה'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הקוד הבא עדיין בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (אופציונלי)
let page = 987 // Int |  (אופציונלי)
let search = "search_example" // String |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]