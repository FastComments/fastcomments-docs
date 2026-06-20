## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | נתיב | כן |  |
| postId | string | נתיב | כן |  |
| broadcastId | string | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_feed_post_public_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-DeleteFeedPostPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postId := "postId_example" // string | 
	broadcastId := "broadcastId_example" // string |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteFeedPostPublic(context.Background(), tenantId, postId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`DeleteFeedPostPublic`: DeleteFeedPostPublicResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteFeedPostPublic`: %v\n", resp)
}
[inline-code-end]

---