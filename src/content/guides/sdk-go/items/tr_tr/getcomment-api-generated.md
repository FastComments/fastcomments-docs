## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: [`GetComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetComment Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComment(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetComment`'ten gelen yanıt: GetComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComment`: %v\n", resp)
}
[inline-code-end]