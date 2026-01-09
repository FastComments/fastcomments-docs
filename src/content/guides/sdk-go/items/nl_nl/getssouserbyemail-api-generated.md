## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| email | string | path | Ja |  |

## Antwoord

Retourneert: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_user_by_email_api_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetSSOUserByEmail Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	email := "email_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUserByEmail(context.Background(), email).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUserByEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetSSOUserByEmail`: GetSSOUserByEmailAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUserByEmail`: %v\n", resp)
}
[inline-code-end]

---