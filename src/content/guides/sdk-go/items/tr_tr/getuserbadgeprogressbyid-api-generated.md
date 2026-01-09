## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badge_progress_by_id_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetUserBadgeProgressById Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetUserBadgeProgressById(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadgeProgressById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserBadgeProgressById`'den yanıt: GetUserBadgeProgressById200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadgeProgressById`: %v\n", resp)
}
[inline-code-end]