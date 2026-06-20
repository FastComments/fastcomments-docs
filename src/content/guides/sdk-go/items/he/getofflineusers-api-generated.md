מי שהגיבו בעבר בעמוד שאינם מחוברים כרגע. ממוינים לפי displayName.
השתמש בזה לאחר שתרוקנו את /users/online כדי להציג מדור "חברים".
דפי תוצאות עם מצביע על commenterName: השרת סורק את האינדקס החלקי {tenantId, urlId, commenterName}
מהערך afterName קדימה באמצעות $gt — ללא עלות של $skip.

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של הדף (מטוהר בצד השרת). |
| afterName | string | query | No | מצביע: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | שובר שוויון של המצביע: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להחמצת פריטים. |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל־GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | מזהה URL של הדף (מטוהר בצד השרת).
	afterName := "afterName_example" // string | מצביע: העבר את nextAfterName מהתגובה הקודמת. (אופציונלי)
	afterUserId := "afterUserId_example" // string | שובר שוויון של המצביע: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להחמצת פריטים. (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]