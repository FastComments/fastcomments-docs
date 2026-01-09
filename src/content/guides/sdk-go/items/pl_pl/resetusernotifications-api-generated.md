## Parametry

| Name | Type | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | zapytanie | Tak |  |
| afterId | string | zapytanie | Nie |  |
| afterCreatedAt | integer | zapytanie | Nie |  |
| unreadOnly | boolean | zapytanie | Nie |  |
| dmOnly | boolean | zapytanie | Nie |  |
| noDm | boolean | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład ResetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (opcjonalne)
	afterCreatedAt := int64(789) // int64 |  (opcjonalne)
	unreadOnly := true // bool |  (opcjonalne)
	dmOnly := true // bool |  (opcjonalne)
	noDm := true // bool |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `ResetUserNotifications`: ResetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]