## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_user_200_response.go)

## Примјер

[inline-code-attrs-start title = 'Примјер GetTenantUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetTenantUser(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetTenantUser`: GetTenantUser200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantUser`: %v\n", resp)
}
[inline-code-end]