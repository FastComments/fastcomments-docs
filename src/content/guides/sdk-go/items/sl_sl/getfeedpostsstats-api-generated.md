## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| postIds | array | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_stats_200_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetFeedPostsStats'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (izbirno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsStats(context.Background(), tenantId).PostIds(postIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsStats``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetFeedPostsStats`: GetFeedPostsStats200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsStats`: %v\n", resp)
}
[inline-code-end]