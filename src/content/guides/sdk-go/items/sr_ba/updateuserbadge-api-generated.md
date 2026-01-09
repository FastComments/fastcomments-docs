## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_update_user_badge_200_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer UpdateUserBadge'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateUserBadgeParams := *openapiclient.NewUpdateUserBadgeParams() // UpdateUserBadgeParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateUserBadge(context.Background(), id).TenantId(tenantId).UpdateUserBadgeParams(updateUserBadgeParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `UpdateUserBadge`: UpdateUserBadge200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateUserBadge`: %v\n", resp)
}
[inline-code-end]