## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_public_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'UpdateFeedPostPublic Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postId := "postId_example" // string | 
	updateFeedPostParams := *openapiclient.NewUpdateFeedPostParams() // UpdateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (optioneel)
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateFeedPostPublic(context.Background(), tenantId, postId).UpdateFeedPostParams(updateFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `UpdateFeedPostPublic`: CreateFeedPostPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateFeedPostPublic`: %v\n", resp)
}
[inline-code-end]