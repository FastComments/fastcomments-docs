## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Example

[inline-code-attrs-start title = 'UpdateModerator Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateModeratorBody := *openapiclient.NewUpdateModeratorBody() // UpdateModeratorBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateModerator(context.Background(), id).TenantId(tenantId).UpdateModeratorBody(updateModeratorBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateModerator``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateModerator`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateModerator`: %v\n", resp)
}
[inline-code-end]
