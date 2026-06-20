## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Örnek

[inline-code-attrs-start title = 'PostRestoreDeletedComment Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.ModerationAPI.PostRestoreDeletedComment(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostRestoreDeletedComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostRestoreDeletedComment`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostRestoreDeletedComment`: %v\n", resp)
}
[inline-code-end]

---