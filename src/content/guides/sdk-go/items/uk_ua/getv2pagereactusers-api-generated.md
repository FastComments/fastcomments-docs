## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| id | string | query | Так |  |

## Відповідь

Повертає: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_v2_page_react_users_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetV2PageReactUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetV2PageReactUsers(context.Background(), tenantId).UrlId(urlId).Id(id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetV2PageReactUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetV2PageReactUsers`: GetV2PageReactUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetV2PageReactUsers`: %v\n", resp)
}
[inline-code-end]