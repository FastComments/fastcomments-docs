## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentIds | string | query | Sì | Un elenco di ID dei commenti separati da virgola. |
| sso | string | query | No |  |

## Risposta

Restituisce: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_checked_comments_for_blocked_200_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di CheckedCommentsForBlocked'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentIds := "commentIds_example" // string | Un elenco di ID dei commenti separati da virgola.
	sso := "sso_example" // string |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CheckedCommentsForBlocked(context.Background()).TenantId(tenantId).CommentIds(commentIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CheckedCommentsForBlocked``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CheckedCommentsForBlocked`: CheckedCommentsForBlocked200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CheckedCommentsForBlocked`: %v\n", resp)
}
[inline-code-end]

---