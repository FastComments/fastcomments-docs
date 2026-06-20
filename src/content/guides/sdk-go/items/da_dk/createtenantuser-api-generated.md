## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_user_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på CreateTenantUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createTenantUserBody := *openapiclient.NewCreateTenantUserBody("Username_example", "Email_example") // CreateTenantUserBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenantUser(context.Background()).TenantId(tenantId).CreateTenantUserBody(createTenantUserBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `CreateTenantUser`: CreateTenantUserResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenantUser`: %v\n", resp)
}
[inline-code-end]

---