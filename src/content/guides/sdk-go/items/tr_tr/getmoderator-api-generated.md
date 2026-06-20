## Parametreler

| Name | Type | Location | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Response

Döndürür: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_moderator_response.go)

## Örnek

[inline-code-attrs-start title = 'GetModerator Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetModerator(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetModerator``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetModerator`'den yanıt: GetModeratorResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetModerator`: %v\n", resp)
}
[inline-code-end]