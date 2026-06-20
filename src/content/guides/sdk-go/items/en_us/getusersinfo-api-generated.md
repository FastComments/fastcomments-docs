Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.
Used by the comment widget to enrich users that just appeared via a presence event.
No page context: privacy is enforced uniformly (private profiles are masked).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Comma-delimited userIds. |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## Example

[inline-code-attrs-start title = 'GetUsersInfo Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | Comma-delimited userIds.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]
