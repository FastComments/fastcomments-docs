## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentId | string | path | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_block_from_comment_public_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład BlockFromCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	publicBlockFromCommentParams := *openapiclient.NewPublicBlockFromCommentParams([]string{"CommentIds_example"}) // PublicBlockFromCommentParams | 
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.BlockFromCommentPublic(context.Background(), commentId).TenantId(tenantId).PublicBlockFromCommentParams(publicBlockFromCommentParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.BlockFromCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `BlockFromCommentPublic`: BlockFromCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.BlockFromCommentPublic`: %v\n", resp)
}
[inline-code-end]

---