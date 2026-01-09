## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| badgeId | string | query | 否 |  |
| type | number | query | 否 |  |
| displayedOnComments | boolean | query | 否 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_badges_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetUserBadges 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (可選)
	badgeId := "badgeId_example" // string |  (可選)
	type_ := float64(1.2) // float64 |  (可選)
	displayedOnComments := true // bool |  (可選)
	limit := float64(1.2) // float64 |  (可選)
	skip := float64(1.2) // float64 |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUserBadges(context.Background()).TenantId(tenantId).UserId(userId).BadgeId(badgeId).Type_(type_).DisplayedOnComments(displayedOnComments).Limit(limit).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUserBadges``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `GetUserBadges` 的回應: GetUserBadges200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUserBadges`: %v\n", resp)
}
[inline-code-end]

---