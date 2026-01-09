## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_badge_200_response.go)

## Örnek

[inline-code-attrs-start title = 'DeleteUserBadge Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.DeleteUserBadge(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteUserBadge`'den yanıt: UpdateUserBadge200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteUserBadge`: %v\n", resp)
}
[inline-code-end]

---