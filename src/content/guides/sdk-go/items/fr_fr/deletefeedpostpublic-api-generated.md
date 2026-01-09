## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_feed_post_public_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple DeleteFeedPostPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.DeleteFeedPostPublic(context.Background(), tenantId, postId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.DeleteFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `DeleteFeedPostPublic`: DeleteFeedPostPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.DeleteFeedPostPublic`: %v\n", resp)
}
[inline-code-end]

---