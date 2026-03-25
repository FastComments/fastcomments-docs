## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| id | string | yol | Evet |  |
| userId | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_subscription_api_response.go)

## Örnek

[inline-code-attrs-start title = 'UpdateSubscription Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateAPIUserSubscriptionData := *openapiclient.NewUpdateAPIUserSubscriptionData() // UpdateAPIUserSubscriptionData | 
	userId := "userId_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateSubscription(context.Background(), id).TenantId(tenantId).UpdateAPIUserSubscriptionData(updateAPIUserSubscriptionData).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateSubscription`'ten gelen yanıt: UpdateSubscriptionAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateSubscription`: %v\n", resp)
}
[inline-code-end]

---