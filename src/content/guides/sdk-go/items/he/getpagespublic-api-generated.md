רשימת עמודים עבור שוכר. משמשת את לקוח שולחן העבודה של FChat למילוי רשימת החדרים שלו.
נדרש ש-`enableFChat` יהיה true על ה-resolved custom config עבור כל עמוד.
עמודים שדורשים SSO מסוננים לפי גישת הקבוצות של המשתמש המבקש.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| cursor | string | query | לא | מחוון עימוד אטום המוחזר כ-`nextCursor` מבקשה קודמת. מקושר לאותו `sortBy`. |
| limit | integer | query | לא | 1..200, default 50 |
| q | string | query | לא | סינון אופציונלי לפי קידומת כותרת, ללא תלות באותיות גדולות/קטנות. |
| sortBy | string | query | לא | סדר מיון. `updatedAt` (ברירת מחדל, האחרונים ראשונים), `commentCount` (העמודים עם הכי הרבה תגובות ראשונים), או `title` (אלפביתי). |
| hasComments | boolean | query | לא | אם true, החזר רק עמודים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	cursor := "cursor_example" // string | מחוון עימוד אטום המוחזר כ-`nextCursor` מבקשה קודמת. מקושר לאותו `sortBy`. (אופציונלי)
	limit := int32(56) // int32 | 1..200, ברירת מחדל 50 (אופציונלי)
	q := "q_example" // string | סינון אופציונלי לפי קידומת כותרת ללא תלות באותיות גדולות/קטנות. (אופציונלי)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, האחרונים ראשונים), `commentCount` (העמודים עם הכי הרבה תגובות ראשונים), או `title` (אלפביתי). (אופציונלי)
	hasComments := true // bool | אם true, החזר רק עמודים עם לפחות תגובה אחת. (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]

---