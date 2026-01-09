## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_user_badge_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα CreateUserBadge'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createUserBadgeParams := *openapiclient.NewCreateUserBadgeParams("UserId_example", "BadgeId_example") // CreateUserBadgeParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateUserBadge(context.Background()).TenantId(tenantId).CreateUserBadgeParams(createUserBadgeParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateUserBadge``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `CreateUserBadge`: CreateUserBadge200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateUserBadge`: %v\n", resp)
}
[inline-code-end]