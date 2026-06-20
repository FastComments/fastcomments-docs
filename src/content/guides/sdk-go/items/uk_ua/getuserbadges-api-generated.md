## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| badgeId | string | query | Ні |  |
| type | number | query | Ні |  |
| displayedOnComments | boolean | query | Ні |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_user_badges_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetUserBadges'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (необов'язково)
	badgeId := "badgeId_example" // string |  (необов'язково)
	type_ := float64(1.2) // float64 |  (необов'язково)
	displayedOnComments := true // bool |  (необов'язково)
	limit := float64(1.2) // float64 |  (необов'язково)
	skip := float64(1.2) // float64 |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetUserBadges`: APIGetUserBadgesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]