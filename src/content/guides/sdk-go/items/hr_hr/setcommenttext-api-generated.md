## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| commentId | string | putanja | Da |  |
| broadcastId | string | upit | Da |  |
| editKey | string | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_200_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer SetCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string | 
	commentTextUpdateRequest := *openapiclient.NewCommentTextUpdateRequest("Comment_example") // CommentTextUpdateRequest | 
	editKey := "editKey_example" // string |  (neobavezno)
	sso := "sso_example" // string |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SetCommentText(context.Background(), tenantId, commentId).BroadcastId(broadcastId).CommentTextUpdateRequest(commentTextUpdateRequest).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `SetCommentText`: SetCommentText200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SetCommentText`: %v\n", resp)
}
[inline-code-end]