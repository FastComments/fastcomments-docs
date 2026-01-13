## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| urlId | string | query | לא |  |
| fromCommentId | string | query | לא |  |
| viewed | boolean | query | לא |  |
| type | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_notifications_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (אופציונלי)
	urlId := "urlId_example" // string |  (אופציונלי)
	fromCommentId := "fromCommentId_example" // string |  (אופציונלי)
	viewed := true // bool |  (אופציונלי)
	type_ := "type__example" // string |  (אופציונלי)
	skip := float64(1.2) // float64 |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetNotifications(context.Background()).TenantId(tenantId).UserId(userId).UrlId(urlId).FromCommentId(fromCommentId).Viewed(viewed).Type_(type_).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`GetNotifications`: GetNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetNotifications`: %v\n", resp)
}
[inline-code-end]