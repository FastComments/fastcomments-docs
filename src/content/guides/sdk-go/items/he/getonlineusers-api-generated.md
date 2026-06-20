צופים שמחוברים כרגע לעמוד: אנשים שה-session של ה-websocket שלהם מנוי לעמוד כרגע.
מחזיר את anonCount + totalCount (מנויים ברחבי החדר, כולל צופים אנונימיים שאותם איננו מפרטים).

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## תגובה

מחזירה: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | מזהה URL של העמוד (נוקה בצד השרת).
	afterName := "afterName_example" // string | Cursor: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
	afterUserId := "afterUserId_example" // string | Cursor tiebreaker: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות זהים לא יגרמו לאיבוד רשומות. (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תשובה מ־`GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]

---