req
tenantId
urlId
userIdWS

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| userIdWS | string | query | Oui |  |
| startTime | integer | query | Oui |  |
| endTime | integer | query | Oui |  |

## Réponse

Renvoie : [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_event_log_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetGlobalEventLog'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	userIdWS := "userIdWS_example" // string | 
	startTime := int64(789) // int64 | 
	endTime := int64(789) // int64 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetGlobalEventLog(context.Background(), tenantId).UrlId(urlId).UserIdWS(userIdWS).StartTime(startTime).EndTime(endTime).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetGlobalEventLog``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetGlobalEventLog`: GetEventLog200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetGlobalEventLog`: %v\n", resp)
}
[inline-code-end]