## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| dir | integer | query | Yes |  |
| sso | string | query | No |  |

## Response

Returns: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_vote_user_names_200_response.go)

## Example

[inline-code-attrs-start title = 'GetCommentVoteUserNames Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	dir := int32(56) // int32 | 
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentVoteUserNames(context.Background(), tenantId, commentId).Dir(dir).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentVoteUserNames``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetCommentVoteUserNames`: GetCommentVoteUserNames200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentVoteUserNames`: %v\n", resp)
}
[inline-code-end]