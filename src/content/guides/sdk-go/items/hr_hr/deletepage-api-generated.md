## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_page_api_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer DeletePage'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.DeletePage(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeletePage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `DeletePage`: DeletePageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeletePage`: %v\n", resp)
}
[inline-code-end]

---