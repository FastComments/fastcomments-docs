## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| urlIdWS | string | שאילתה | כן |  |
| userIds | string | שאילתה | כן |  |

## תגובה

מחזיר: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_presence_statuses_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל־GetUserPresenceStatuses'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlIdWS := "urlIdWS_example" // string | 
	userIds := "userIds_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserPresenceStatuses(context.Background()).TenantId(tenantId).UrlIdWS(urlIdWS).UserIds(userIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserPresenceStatuses``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`GetUserPresenceStatuses`: GetUserPresenceStatusesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserPresenceStatuses`: %v\n", resp)
}
[inline-code-end]