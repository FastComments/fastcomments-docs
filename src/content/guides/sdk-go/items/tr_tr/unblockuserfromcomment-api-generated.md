## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |

## Yanıt

Döndürür: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_un_block_comment_public_200_response.go)

## Örnek

[inline-code-attrs-start title = 'UnBlockUserFromComment Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	unBlockFromCommentParams := *openapiclient.NewUnBlockFromCommentParams() // UnBlockFromCommentParams | 
	userId := "userId_example" // string |  (isteğe bağlı)
	anonUserId := "anonUserId_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnBlockUserFromComment(context.Background(), id).TenantId(tenantId).UnBlockFromCommentParams(unBlockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnBlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UnBlockUserFromComment`'dan dönen yanıt: UnBlockCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnBlockUserFromComment`: %v\n", resp)
}
[inline-code-end]