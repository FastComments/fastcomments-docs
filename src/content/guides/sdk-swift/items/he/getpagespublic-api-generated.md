---
רשימת דפים עבור דייר. משמש את לקוח השולחן העבודה של FChat למלא את רשימת החדרים שלו.  
דורש ש-`enableFChat` יהיה `true` בתצורה המותאמת המוגדרת לכל דף.  
דפים הדורשים SSO מסוננים על פי הגישה לקבוצת המשתמש שמבצע את הבקשה.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | סמן דפדוף בלתי ניראה שמוחזר כ-`nextCursor` מבקשה קודמת. מקושר לאותו `sortBy`. |
| limit | integer | query | No | 1..200, ברירת מחדל 50 |
| q | string | query | No | מסנן קידומת כותרת רגישות לאותיות, אופציונלי. |
| sortBy | string | query | No | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (התגובות הרבות ראשון), או `title` (בסדר אלפביתי). |
| hasComments | boolean | query | No | אם אמת, תחזיר רק דפים שיש להם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמת הקוד הבאה עדיין ב-beta. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | סמן דפדוף בלתי ניראה שמוחזר כ-`nextCursor` מבקשה קודמת. מקושר לאותו `sortBy`. (optional)
let limit = 987 // Int | 1..200, ברירת מחדל 50 (optional)
let q = "q_example" // String | מסנן קידומת כותרת רגישות לאותיות, אופציונלי. (optional)
let sortBy = PagesSortBy() // PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדש ראשון), `commentCount` (התגובות הרבות ראשון), או `title` (בסדר אלפביתי). (optional)
let hasComments = true // Bool | אם אמת, תחזיר רק דפים עם לפחות תגובה אחת. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
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