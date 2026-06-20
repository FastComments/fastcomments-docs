## Parametreler

| İsim | Tip | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tag | string | path | Evet |  |
| tenantId | string | query | Hayır |  |

## Yanıt

Döndürür: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_hash_tag_response.go)

## Örnek

[inline-code-attrs-start title = 'PatchHashTag Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tag := "tag_example" // string | 
	tenantId := "tenantId_example" // string |  (isteğe bağlı)
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PatchHashTag`'den gelen yanıt: UpdateHashTagResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]