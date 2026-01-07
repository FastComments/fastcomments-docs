## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| afterId | string | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_200_response.go)

## Example

[inline-code-attrs-start title = 'ResetUserNotifications Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (optional)
	afterCreatedAt := int64(789) // int64 |  (optional)
	unreadOnly := true // bool |  (optional)
	dmOnly := true // bool |  (optional)
	noDm := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ResetUserNotifications`: ResetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]
