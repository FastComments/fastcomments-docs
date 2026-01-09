## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| badgeId | string | query | לא |  |
| type | number | query | לא |  |
| displayedOnComments | boolean | query | לא |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badges_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetUserBadges'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (אופציונלי)
	badgeId := "badgeId_example" // string |  (אופציונלי)
	type_ := float64(1.2) // float64 |  (אופציונלי)
	displayedOnComments := true // bool |  (אופציונלי)
	limit := float64(1.2) // float64 |  (אופציונלי)
	skip := float64(1.2) // float64 |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetUserBadges`: GetUserBadges200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]