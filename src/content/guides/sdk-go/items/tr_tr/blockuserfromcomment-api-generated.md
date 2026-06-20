---
## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |

## Yanıt

Döndürür: [`BlockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_block_success.go)

## Örnek

[inline-code-attrs-start title = 'BlockUserFromComment Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	blockFromCommentParams := *openapiclient.NewBlockFromCommentParams() // BlockFromCommentParams | 
	userId := "userId_example" // string |  (isteğe bağlı)
	anonUserId := "anonUserId_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.BlockUserFromComment(context.Background(), id).TenantId(tenantId).BlockFromCommentParams(blockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.BlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `BlockUserFromComment`'dan gelen yanıt: BlockSuccess
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.BlockUserFromComment`: %v\n", resp)
}
[inline-code-end]

---