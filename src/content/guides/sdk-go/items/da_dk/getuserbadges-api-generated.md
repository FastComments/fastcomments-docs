## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| badgeId | string | query | Nej |  |
| type | number | query | Nej |  |
| displayedOnComments | boolean | query | Nej |  |
| limit | number | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badges_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetUserBadges Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (valgfri)
	badgeId := "badgeId_example" // string |  (valgfri)
	type_ := float64(1.2) // float64 |  (valgfri)
	displayedOnComments := true // bool |  (valgfri)
	limit := float64(1.2) // float64 |  (valgfri)
	skip := float64(1.2) // float64 |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetUserBadges`: GetUserBadges200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]