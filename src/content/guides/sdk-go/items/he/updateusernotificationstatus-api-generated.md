---
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| notificationId | string | path | כן |  |
| newStatus | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_notification_status_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-UpdateUserNotificationStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	notificationId := "notificationId_example" // string | 
	newStatus := "newStatus_example" // string | 
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateUserNotificationStatus(context.Background(), notificationId, newStatus).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateUserNotificationStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`UpdateUserNotificationStatus`: UpdateUserNotificationStatus200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateUserNotificationStatus`: %v\n", resp)
}
[inline-code-end]

---