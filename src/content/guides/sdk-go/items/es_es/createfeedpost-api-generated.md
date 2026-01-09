## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## Respuesta

Devuelve: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de CreateFeedPost'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string |  (opcional)
	isLive := true // bool |  (opcional)
	doSpamCheck := true // bool |  (opcional)
	skipDupCheck := true // bool |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateFeedPost(context.Background()).TenantId(tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).IsLive(isLive).DoSpamCheck(doSpamCheck).SkipDupCheck(skipDupCheck).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateFeedPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `CreateFeedPost`: CreateFeedPost200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateFeedPost`: %v\n", resp)
}
[inline-code-end]