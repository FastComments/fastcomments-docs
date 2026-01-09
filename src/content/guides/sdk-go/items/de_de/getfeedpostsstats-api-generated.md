## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postIds | array | query | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_stats_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetFeedPostsStats'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postIds := []string{"Inner_example"} // []string | 
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsStats(context.Background(), tenantId).PostIds(postIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsStats``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetFeedPostsStats`: GetFeedPostsStats200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsStats`: %v\n", resp)
}
[inline-code-end]