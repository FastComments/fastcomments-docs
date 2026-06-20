## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| badgeId | string | query | Ne |  |
| type | number | query | Ne |  |
| displayedOnComments | boolean | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrača: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badges_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetUserBadges'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (neobvezno)
	badgeId := "badgeId_example" // string |  (neobvezno)
	type_ := float64(1.2) // float64 |  (neobvezno)
	displayedOnComments := true // bool |  (neobvezno)
	limit := float64(1.2) // float64 |  (neobvezno)
	skip := float64(1.2) // float64 |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetUserBadges`: APIGetUserBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]

---