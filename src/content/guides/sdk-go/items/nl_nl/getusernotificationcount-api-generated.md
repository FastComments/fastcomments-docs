## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notification_count_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van GetUserNotificationCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotificationCount(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetUserNotificationCount`: GetUserNotificationCount200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotificationCount`: %v\n", resp)
}
[inline-code-end]