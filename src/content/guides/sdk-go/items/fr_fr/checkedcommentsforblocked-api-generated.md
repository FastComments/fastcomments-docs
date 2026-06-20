---
## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentIds | string | query | Oui | Une liste d'identifiants de commentaires séparés par des virgules. |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_check_blocked_comments_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de CheckedCommentsForBlocked'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentIds := "commentIds_example" // string | Une liste d'identifiants de commentaires séparés par des virgules.
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CheckedCommentsForBlocked(context.Background()).TenantId(tenantId).CommentIds(commentIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CheckedCommentsForBlocked``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `CheckedCommentsForBlocked`: CheckBlockedCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CheckedCommentsForBlocked`: %v\n", resp)
}
[inline-code-end]

---