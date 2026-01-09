## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| limit | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badge_progress_list_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetUserBadgeProgressList Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optional)
	limit := float64(1.2) // float64 |  (optional)
	skip := float64(1.2) // float64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressList(context.Background()).TenantId(tenantId).UserId(userId).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetUserBadgeProgressList`: GetUserBadgeProgressList200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressList`: %v\n", resp)
}
[inline-code-end]

---