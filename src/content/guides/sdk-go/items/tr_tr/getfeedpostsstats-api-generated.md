## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| postIds | array | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_feed_posts_stats_response.go)

## Örnek

[inline-code-attrs-start title = 'GetFeedPostsStats Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postIds := []string{"Inner_example"} // []string | 
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsStats(context.Background(), tenantId).PostIds(postIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsStats``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetFeedPostsStats`'den gelen yanıt: FeedPostsStatsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsStats`: %v\n", resp)
}
[inline-code-end]