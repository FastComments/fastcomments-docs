---
רשימת דפים עבור דייר. משמשת את לקוח שולחן העבודה של FChat למילוי רשימת החדרים שלו.
נדרש ש-`enableFChat` יהיה true בהגדרת ה-custom שהתקבלה עבור כל דף.
דפים הדורשים SSO מסוננים מול גישת הקבוצות של המשתמש המבקש.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| cursor | string | query | לא | סמן דפדוף עמום שמוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | לא | 1..200, ברירת מחדל 50 |
| q | string | query | לא | מסנן אופציונלי לפי קידומת הכותרת שאינה רגישה לרישיות. |
| sortBy | string | query | לא | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ראשונים), `commentCount` (הכי הרבה תגובות תחילה), או `title` (לפי סדר אלפאביתי). |
| hasComments | boolean | query | לא | אם true, החזר רק דפים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. עבור כל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | סמן דפדוף עמום שמוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. (אופציונלי)
let limit = 987 // Int | 1..200, ברירת מחדל 50 (אופציונלי)
let q = "q_example" // String | מסנן אופציונלי לפי קידומת הכותרת שאינה רגישה לרישיות. (אופציונלי)
let sortBy = PagesSortBy() // PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ראשונים), `commentCount` (הכי הרבה תגובות תחילה), או `title` (לפי סדר אלפאביתי). (אופציונלי)
let hasComments = true // Bool | אם true, החזר רק דפים עם לפחות תגובה אחת. (אופציונלי)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
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