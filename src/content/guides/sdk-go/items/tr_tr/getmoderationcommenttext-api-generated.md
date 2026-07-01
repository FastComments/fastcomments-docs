## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_response.go)

## Örnek

[inline-code-attrs-start title = 'GetModerationCommentText Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationCommentText(context.Background(), commentId).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`ModerationAPI.GetModerationCommentText` çağrısı sırasında hata: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// `GetModerationCommentText` API çağrısının yanıtı: GetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Yanıt `ModerationAPI.GetModerationCommentText`'tan: %v\n", resp)
}
[inline-code-end]

---