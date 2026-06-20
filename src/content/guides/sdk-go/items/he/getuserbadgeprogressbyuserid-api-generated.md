## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | path | כן |  |

## תגובה

מחזיר: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badge_progress_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetUserBadgeProgressByUserId'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressByUserId(context.Background(), userId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressByUserId``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserBadgeProgressByUserId`: APIGetUserBadgeProgressResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressByUserId`: %v\n", resp)
}
[inline-code-end]

---