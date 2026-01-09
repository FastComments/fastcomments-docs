req
tenantId
afterId

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Respuesta

Devuelve: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetFeedPosts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (opcional)
	limit := int32(56) // int32 |  (opcional)
	tags := []string{"Inner_example"} // []string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetFeedPosts`: GetFeedPosts200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]