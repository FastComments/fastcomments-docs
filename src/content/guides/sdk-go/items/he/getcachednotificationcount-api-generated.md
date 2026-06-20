## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_cached_notification_count_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetCachedNotificationCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // מחרוזת | 
	id := "id_example" // מחרוזת | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetCachedNotificationCount(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetCachedNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`GetCachedNotificationCount`: GetCachedNotificationCountResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetCachedNotificationCount`: %v\n", resp)
}
[inline-code-end]

---