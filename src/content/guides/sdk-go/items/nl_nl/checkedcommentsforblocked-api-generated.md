---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentIds | string | query | Ja | Een door komma's gescheiden lijst met comment-id's. |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_checked_comments_for_blocked_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'CheckedCommentsForBlocked Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentIds := "commentIds_example" // string | Een door komma's gescheiden lijst met comment-id's.
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CheckedCommentsForBlocked(context.Background()).TenantId(tenantId).CommentIds(commentIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CheckedCommentsForBlocked``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `CheckedCommentsForBlocked`: CheckedCommentsForBlocked200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CheckedCommentsForBlocked`: %v\n", resp)
}
[inline-code-end]

---