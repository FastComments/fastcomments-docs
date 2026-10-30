## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | ה‑tenant לקריאת יומני ביקורת. |
| limit | integer | query | No | כמה יומנים להחזיר. מקסימום 10000, ברירת מחדל 1000. |
| skip | integer | query | No | כמה יומנים לדלג, עבור דפדוף. |
| order | string | query | No | כיוון המיון על זמן האירוע. ברירת מחדל DESC (החדש ביותר ראשון). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | להחזיר רק אירועים שבוצעו על ידי שם משתמש זה. |
| ip | string | query | No | להחזיר רק אירועים מכתובת IP זו. |
| crudType | string | query | No | להחזיר רק אירועים מסוג זה: c, r, u, d, או login. |
| resourceName | string | query | No | להחזיר רק אירועים עבור משאב זה, למשל User או Moderator. |
| targetId | string | query | No | להחזיר רק אירועים שהאובייקט המושפע שלהם בעל מזהה זה. |
| target | string | query | No | התאמה של תת‑מחרוזת ללא תלות ברישיות על תווית האובייקט המושפע, למשל שם משתמש או אימייל. |
| includeManagedTenants | boolean | query | No | להחזיר גם אירועים מדיירים שה‑tenant הזה מנהל. |

## תגובה

מחזיר: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגמי הקוד הבאים עדיין בגרסת בטא. לכל בעיה, אנא דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | ה‑tenant לקריאת יומני ביקורת.
let limit = 987 // Int | כמה יומנים להחזיר. מקסימום 10000, ברירת מחדל 1000. (אופציונלי)
let skip = 987 // Int | כמה יומנים לדלג, עבור דפדוף. (אופציונלי)
let order = SORT_DIR() // SORTDIR | כיוון המיון על זמן האירוע. ברירת מחדל DESC (החדש ביותר ראשון). (אופציונלי)
let after = 987 // Int64 |  (אופציונלי)
let before = 987 // Int64 |  (אופציונלי)
let username = "username_example" // String | להחזיר רק אירועים שבוצעו על ידי שם משתמש זה. (אופציונלי)
let ip = "ip_example" // String | להחזיר רק אירועים מכתובת IP זו. (אופציונלי)
let crudType = "crudType_example" // String | להחזיר רק אירועים מסוג זה: c, r, u, d, או login. (אופציונלי)
let resourceName = "resourceName_example" // String | להחזיר רק אירועים עבור משאב זה, למשל User או Moderator. (אופציונלי)
let targetId = "targetId_example" // String | להחזיר רק אירועים שהאובייקט המושפע שלהם בעל מזהה זה. (אופציונלי)
let target = "target_example" // String | התאמה של תת‑מחרוזת ללא תלות ברישיות על תווית האובייקט המושפע, למשל שם משתמש או אימייל. (אופציונלי)
let includeManagedTenants = true // Bool | להחזיר גם אירועים מדיירים שה‑tenant הזה מנהל. (אופציונלי)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]