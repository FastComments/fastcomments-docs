## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple PostSetCommentReviewStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	reviewed := true // bool | (optionnel)
	broadcastId := "broadcastId_example" // string | (optionnel)
	sso := "sso_example" // string | (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentReviewStatus(context.Background(), commentId).TenantId(tenantId).Reviewed(reviewed).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel `ModerationAPI.PostSetCommentReviewStatus`` : %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète : %v\n", r)
	}
	// réponse de `PostSetCommentReviewStatus` : APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.PostSetCommentReviewStatus` : %v\n", resp)
}
[inline-code-end]