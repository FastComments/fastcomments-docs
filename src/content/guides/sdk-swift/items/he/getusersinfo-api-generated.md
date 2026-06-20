מידע מרוכז על משתמשים עבור שוכר. בהינתן userIds, מחזיר מידע תצוגה מ-User / SSOUser. מנוצל על ידי וידג'ט התגובות להעשיר משתמשים שהופיעו זה עתה דרך אירוע נוכחות. אין הקשר של דף: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| ids | string | query | כן | רשימת userIds מופרדת בפסיקים. |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUsersInfo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, אנא דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | רשימת userIds מופרדת בפסיקים.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]