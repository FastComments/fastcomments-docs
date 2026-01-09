## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Antwort

Gibt zurück: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_subscription_api_response.go)

## Beispiel

[inline-code-attrs-start title = 'DeleteSubscription Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteSubscription(context.Background(), id).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `DeleteSubscription`: DeleteSubscriptionAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteSubscription`: %v\n", resp)
}
[inline-code-end]

---