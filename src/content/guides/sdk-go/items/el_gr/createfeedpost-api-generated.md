## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| broadcastId | string | query | Όχι |  |
| isLive | boolean | query | Όχι |  |
| doSpamCheck | boolean | query | Όχι |  |
| skipDupCheck | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα CreateFeedPost'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createFeedPostParams := *openapiclient.NewCreateFeedPostParams() // CreateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (προαιρετικό)
	isLive := true // bool |  (προαιρετικό)
	doSpamCheck := true // bool |  (προαιρετικό)
	skipDupCheck := true // bool |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateFeedPost(context.Background()).TenantId(tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).IsLive(isLive).DoSpamCheck(doSpamCheck).SkipDupCheck(skipDupCheck).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateFeedPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `CreateFeedPost`: CreateFeedPost200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateFeedPost`: %v\n", resp)
}
[inline-code-end]