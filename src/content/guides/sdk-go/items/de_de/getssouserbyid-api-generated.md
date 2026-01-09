## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_user_by_id_api_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetSSOUserById Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetSSOUserById(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUserById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetSSOUserById`: GetSSOUserByIdAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUserById`: %v\n", resp)
}
[inline-code-end]

---