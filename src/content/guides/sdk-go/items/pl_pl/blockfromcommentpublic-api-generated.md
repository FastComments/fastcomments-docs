## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |
| commentId | string | ścieżka | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`BlockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_block_success.go)

## Przykład

[inline-code-attrs-start title = 'Przykład BlockFromCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	publicBlockFromCommentParams := *openapiclient.NewPublicBlockFromCommentParams([]string{"CommentIds_example"}) // PublicBlockFromCommentParams | 
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.BlockFromCommentPublic(context.Background(), commentId).TenantId(tenantId).PublicBlockFromCommentParams(publicBlockFromCommentParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.BlockFromCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `BlockFromCommentPublic`: BlockSuccess
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.BlockFromCommentPublic`: %v\n", resp)
}
[inline-code-end]

---