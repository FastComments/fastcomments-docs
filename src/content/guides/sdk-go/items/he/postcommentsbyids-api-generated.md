## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Response

מחזיר: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_child_comments_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת PostCommentsByIds'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentsByIdsParams := *openapiclient.NewCommentsByIdsParams([]string{"Ids_example"}) // CommentsByIdsParams | 
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostCommentsByIds(context.Background()).TenantId(tenantId).CommentsByIdsParams(commentsByIdsParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostCommentsByIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`PostCommentsByIds`: ModerationAPIChildCommentsResponse
	fmt.Fprintf(os.Stdout, "תגובה מ-`ModerationAPI.PostCommentsByIds`: %v\n", resp)
}
[inline-code-end]