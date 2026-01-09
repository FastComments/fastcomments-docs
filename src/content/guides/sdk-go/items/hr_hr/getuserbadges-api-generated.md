## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| badgeId | string | query | Ne |  |
| type | number | query | Ne |  |
| displayedOnComments | boolean | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badges_200_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetUserBadges'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (neobavezno)
	badgeId := "badgeId_example" // string |  (neobavezno)
	type_ := float64(1.2) // float64 |  (neobavezno)
	displayedOnComments := true // bool |  (neobavezno)
	limit := float64(1.2) // float64 |  (neobavezno)
	skip := float64(1.2) // float64 |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetUserBadges`: GetUserBadges200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]