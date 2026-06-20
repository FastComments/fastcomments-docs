## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| badgeId | string | query | Non |  |
| type | number | query | Non |  |
| displayedOnComments | boolean | query | Non |  |
| limit | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie : [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badges_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetUserBadges'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (facultatif)
	badgeId := "badgeId_example" // string |  (facultatif)
	type_ := float64(1.2) // float64 |  (facultatif)
	displayedOnComments := true // bool |  (facultatif)
	limit := float64(1.2) // float64 |  (facultatif)
	skip := float64(1.2) // float64 |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetUserBadges`: APIGetUserBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]

---