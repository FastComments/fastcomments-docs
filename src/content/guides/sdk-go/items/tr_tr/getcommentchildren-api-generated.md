## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_child_comments_response.go)

## Örnek

[inline-code-attrs-start title = 'GetCommentChildren Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCommentChildren(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCommentChildren``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentChildren`'den gelen yanıt: ModerationAPIChildCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCommentChildren`: %v\n", resp)
}
[inline-code-end]

---