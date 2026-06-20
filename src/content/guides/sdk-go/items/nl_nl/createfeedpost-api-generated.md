## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| broadcastId | string | query | Nee |  |
| isLive | boolean | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| skipDupCheck | boolean | query | Nee |  |

## Respons

Retourneert: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_posts_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'CreateFeedPost Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createFeedPostParams := *openapiclient.NewCreateFeedPostParams() // CreateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (optioneel)
	isLive := true // bool |  (optioneel)
	doSpamCheck := true // bool |  (optioneel)
	skipDupCheck := true // bool |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateFeedPost(context.Background()).TenantId(tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).IsLive(isLive).DoSpamCheck(doSpamCheck).SkipDupCheck(skipDupCheck).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateFeedPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `CreateFeedPost`: CreateFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateFeedPost`: %v\n", resp)
}
[inline-code-end]