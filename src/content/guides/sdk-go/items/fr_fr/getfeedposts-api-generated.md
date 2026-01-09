req
tenantId
afterId

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |

## Réponse

Renvoie: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetFeedPosts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (optionnel)
	limit := int32(56) // int32 |  (optionnel)
	tags := []string{"Inner_example"} // []string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetFeedPosts`: GetFeedPosts200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]