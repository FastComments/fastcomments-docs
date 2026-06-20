## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | string | query | Nee |  |

## Respons

Geeft terug: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld ReplaceTenantUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	replaceTenantUserBody := *openapiclient.NewReplaceTenantUserBody("Username_example", "Email_example") // ReplaceTenantUserBody | 
	updateComments := "updateComments_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ReplaceTenantUser(context.Background(), id).TenantId(tenantId).ReplaceTenantUserBody(replaceTenantUserBody).UpdateComments(updateComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ReplaceTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReplaceTenantUser`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ReplaceTenantUser`: %v\n", resp)
}
[inline-code-end]

---