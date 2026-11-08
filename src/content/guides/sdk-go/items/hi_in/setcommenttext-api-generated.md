## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| broadcastId | string | query | हाँ |  |
| editKey | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_api_set_comment_text_response.go)

## उदाहरण

[inline-code-attrs-start title = 'SetCommentText उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	commentTextUpdateRequest := *openapiclient.NewCommentTextUpdateRequest("Comment_example") // CommentTextUpdateRequest | 
	editKey := "editKey_example" // string |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SetCommentText(context.Background(), tenantId, commentId).BroadcastId(broadcastId).CommentTextUpdateRequest(commentTextUpdateRequest).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SetCommentText` से प्रतिक्रिया: PublicAPISetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SetCommentText`: %v\n", resp)
}
[inline-code-end]