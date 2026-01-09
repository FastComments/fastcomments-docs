## Parametre

| Name | Type | Location | Required | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notification_count_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetUserNotificationCount Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotificationCount(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetUserNotificationCount`: GetUserNotificationCount200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotificationCount`: %v\n", resp)
}
[inline-code-end]