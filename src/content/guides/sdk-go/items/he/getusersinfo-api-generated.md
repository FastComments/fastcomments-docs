מידע אצווה של משתמשים עבור tenant. בהינתן userIds, מחזיר מידע להצגה מ- User / SSOUser.
משמש ב-widget של תגובות כדי להעשיר משתמשים שזה עתה הופיעו באמצעות אירוע נוכחות.
ללא הקשר של דף: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| ids | string | query | כן | רשימת userIds מופרדת בפסיקים. |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של GetUsersInfo'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | מזהי userIds מופרדים בפסיקים.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]