---
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| userId | string | query | לא |  |
| direction | string | query | לא |  |
| repliesToUserId | string | query | לא |  |
| page | number | query | לא |  |
| includei10n | boolean | query | לא |  |
| locale | string | query | לא |  |
| isCrawler | boolean | query | לא |  |

## תגובה

מחזיר: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות בקוד הבאות עדיין בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (אופציונלי)
let direction = SortDirections() // SortDirections |  (אופציונלי)
let repliesToUserId = "repliesToUserId_example" // String |  (אופציונלי)
let page = 987 // Double |  (אופציונלי)
let includei10n = true // Bool |  (אופציונלי)
let locale = "locale_example" // String |  (אופציונלי)
let isCrawler = true // Bool |  (אופציונלי)

PublicAPI.getCommentsForUser(options: PublicAPI.GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler)) { (response, error) in
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