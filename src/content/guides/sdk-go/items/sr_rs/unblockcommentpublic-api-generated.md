## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | упит | Да |  |
| commentId | string | путања | Да |  |
| sso | string | упит | Не |  |

## Одговор

Враћа: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_unblock_success.go)

## Пример

[inline-code-attrs-start title = 'Пример UnBlockCommentPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UnBlockCommentPublic(context.Background(), commentId).TenantId(tenantId).PublicBlockFromCommentParams(publicBlockFromCommentParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UnBlockCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `UnBlockCommentPublic`: UnblockSuccess
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UnBlockCommentPublic`: %v\n", resp)
}
[inline-code-end]