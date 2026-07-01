## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## Yanıt

Returns: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_hash_tag_response.go)

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
	tenantId := "tenantId_example" // string | 
	tag := "tag_example" // string | 
	updateHashTagBody := *openapiclient.NewUpdateHashTagBody() // UpdateHashTagBody |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchHashTag(context.Background(), tag).TenantId(tenantId).UpdateHashTagBody(updateHashTagBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`DefaultAPI.PatchHashTag` çağrılırken hata oluştu: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// `PatchHashTag` yanıtı: UpdateHashTagResponse
	fmt.Fprintf(os.Stdout, "Yanıt `DefaultAPI.PatchHashTag`: %v\n", resp)
}
[inline-code-end]

---