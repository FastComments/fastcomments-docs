## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_response.go)

## Örnek

[inline-code-attrs-start title = 'UpdateFeedPostPublic Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateFeedPostParams := *openapiclient.NewUpdateFeedPostParams() // UpdateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateFeedPostPublic(context.Background(), tenantId, postId).UpdateFeedPostParams(updateFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateFeedPostPublic`'ten gelen yanıt: CreateFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateFeedPostPublic`: %v\n", resp)
}
[inline-code-end]

---