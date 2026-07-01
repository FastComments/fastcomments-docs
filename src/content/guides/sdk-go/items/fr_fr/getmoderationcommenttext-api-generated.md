## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetModerationCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationCommentText(context.Background(), commentId).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel `ModerationAPI.GetModerationCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète: %v\n", r)
	}
	// réponse de `GetModerationCommentText` : GetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.GetModerationCommentText` : %v\n", resp)
}
[inline-code-end]