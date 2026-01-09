## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_subscription_api_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer CreateSubscription'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createAPIUserSubscriptionData := *openapiclient.NewCreateAPIUserSubscriptionData("UrlId_example") // CreateAPIUserSubscriptionData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateSubscription(context.Background()).TenantId(tenantId).CreateAPIUserSubscriptionData(createAPIUserSubscriptionData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `CreateSubscription`: CreateSubscriptionAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateSubscription`: %v\n", resp)
}
[inline-code-end]

---