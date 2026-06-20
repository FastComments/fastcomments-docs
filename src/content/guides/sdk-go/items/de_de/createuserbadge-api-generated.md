## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_create_user_badge_response.go)

## Beispiel

[inline-code-attrs-start title = 'CreateUserBadge Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createUserBadgeParams := *openapiclient.NewCreateUserBadgeParams("UserId_example", "BadgeId_example") // CreateUserBadgeParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateUserBadge(context.Background()).TenantId(tenantId).CreateUserBadgeParams(createUserBadgeParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `CreateUserBadge`: APICreateUserBadgeResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateUserBadge`: %v\n", resp)
}
[inline-code-end]