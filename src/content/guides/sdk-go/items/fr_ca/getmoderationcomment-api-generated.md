## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| includeEmail | boolean | query | Non |  |
| includeIP | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_comment_response.go)

## Exemple

[inline-code-attrs-start title = 'GetModerationComment Exemple'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeEmail := true // bool |  (facultatif)
	includeIP := true // bool |  (facultatif)
	sso := "sso_example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationComment(context.Background(), commentId).TenantId(tenantId).IncludeEmail(includeEmail).IncludeIP(includeIP).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel `ModerationAPI.GetModerationComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète : %v\n", r)
	}
	// réponse de `GetModerationComment`: ModerationAPICommentResponse
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.GetModerationComment`: %v\n", resp)
}
[inline-code-end]

---